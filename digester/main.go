package main

import (
	"archive/zip"
	"database/sql"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	_ "github.com/lib/pq"
)

type User struct {
	ID      string `json:"id"`
	Name    string `json:"name"`
	Profile struct {
		RealName    string `json:"real_name"`
		DisplayName string `json:"display_name"`
		Email       string `json:"email"`
		ImageURL    string `json:"image_192"`
	} `json:"profile"`
	Deleted bool `json:"deleted"`
	IsBot   bool `json:"is_bot"`
}

type Channel struct {
	Name  string `json:"name"`
	Topic struct {
		Value string `json:"value"`
	} `json:"topic"`
	Purpose struct {
		Value string `json:"value"`
	} `json:"purpose"`
}

type Message struct {
	Channel         string
	SubType         string  `json:"subtype"`
	UserID          string  `json:"user"`
	BotID           string  `json:"bot_id"`
	BotUsername     string  `json:"username"`
	Timestamp       string  `json:"ts"`
	Text            string  `json:"text"`
	ThreadTimestamp string  `json:"thread_ts"`
	ParentUserID    string  `json:"parent_user_id"`
	Blocks          []Block `json:"blocks"`
}

const (
	ZIPFILE_PATH      = "/slack-export.zip"
	EXTRACTION_DIR    = "/extracted"
	USERS_FILEPATH    = EXTRACTION_DIR + "/users.json"
	CHANNELS_FILEPATH = EXTRACTION_DIR + "/channels.json"
	SLACKBOT_ID       = "USLACKBOT"
)

var (
	db       *sql.DB
	users    []User
	channels []Channel
)

func CheckError(err error) {
	if err != nil {
		panic(err)
	}
}

func unzipFile(file *zip.File, dest string) error {
	// Check if file paths are not vulnerable to Zip Slip
	filePath := filepath.Join(dest, file.Name)
	if !strings.HasPrefix(filePath, filepath.Clean(dest)+string(os.PathSeparator)) {
		return fmt.Errorf("%s: illegal file path", filePath)
	}

	if file.FileInfo().IsDir() {
		if err := os.MkdirAll(filePath, os.ModePerm); err != nil {
			return err
		}
		return nil
	}

	if err := os.MkdirAll(filepath.Dir(filePath), os.ModePerm); err != nil {
		return err
	}

	destFile, err := os.OpenFile(filePath, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, file.Mode())
	if err != nil {
		return err
	}
	defer destFile.Close()

	zipFile, err := file.Open()
	if err != nil {
		return err
	}
	defer zipFile.Close()

	if _, err := io.Copy(destFile, zipFile); err != nil {
		return err
	}

	return nil
}

func unzipSource(src, dest string) error {
	reader, err := zip.OpenReader(src)
	if err != nil {
		return err
	}
	defer reader.Close()

	dest, err = filepath.Abs(dest)
	if err != nil {
		return err
	}

	for _, file := range reader.File {
		err := unzipFile(file, dest)
		if err != nil {
			return err
		}
	}

	return nil
}

func main() {
	host := os.Getenv("TUMMY_HOST")
	port, err := strconv.Atoi(os.Getenv("TUMMY_PORT"))
	CheckError(err)
	user := os.Getenv("TUMMY_USERNAME")
	password := os.Getenv("TUMMY_PASSWORD")
	dbname := os.Getenv("TUMMY_DB")

	psqlconn := fmt.Sprintf("host=%s port=%d user=%s password=%s dbname=%s sslmode=disable", host, port, user, password, dbname)

	db, err = sql.Open("postgres", psqlconn)
	CheckError(err)
	defer db.Close()

	err = db.Ping()
	CheckError(err)
	log.Println("Digester is now successfully connected to the tummy!")

	err = unzipSource(ZIPFILE_PATH, EXTRACTION_DIR)
	CheckError(err)
	log.Println("Slack export has been successfully extracted!")

	usersFile, err := os.ReadFile(USERS_FILEPATH)
	CheckError(err)

	err = json.Unmarshal(usersFile, &users)
	CheckError(err)
	log.Println("Digester found " + fmt.Sprint(len(users)) + " users.")

	// Slackbot is a special bot that is not included in the users.json file and also doesnt have a bot_id
	slackbot := User{
		ID:      SLACKBOT_ID,
		Name:    "slackbot",
		Deleted: false,
		IsBot:   true,
	}
	slackbot.Profile.DisplayName = "Slackbot"
	slackbot.Profile.RealName = "Slackbot"
	users = append(users, slackbot)

	userIDSet := make(map[string]bool)

	for _, user := range users {
		query := "INSERT INTO users (id, name, real_name, display_name, email, deleted, is_bot, image_url) VALUES ($1, $2, $3, $4, $5, $6, $7, $8);"
		_, err = db.Exec(query, user.ID, user.Name, user.Profile.RealName, user.Profile.DisplayName, user.Profile.Email, user.Deleted, user.IsBot, user.Profile.ImageURL)
		CheckError(err)
		userIDSet[user.ID] = true
	}
	log.Println("Digester digested users and sent to the tummy.")

	channelsFile, err := os.ReadFile(CHANNELS_FILEPATH)
	CheckError(err)

	err = json.Unmarshal(channelsFile, &channels)
	CheckError(err)
	log.Println("Digester found " + fmt.Sprint(len(channels)) + " channels.")

	for _, channel := range channels {
		query := "INSERT INTO channels (name, topic, purpose) VALUES ($1, $2, $3)"
		_, err = db.Exec(query, channel.Name, channel.Topic.Value, channel.Purpose.Value)
		CheckError(err)
	}
	log.Println("Digester digested channels and sent to the tummy.")

	totalMessagesCount := 0
	totalMessagesAddedCount := 0
	for _, channel := range channels {
		messagesDirPath := filepath.Join(EXTRACTION_DIR, channel.Name)
		messageFiles, err := os.ReadDir(messagesDirPath)
		CheckError(err)

		for _, messageFile := range messageFiles {
			if messageFile.Name() == "canvas_in_the_conversation.json" {
				continue
			}

			messageFilePath := filepath.Join(messagesDirPath, messageFile.Name())
			messagesFile, err := os.ReadFile(messageFilePath)
			CheckError(err)

			var messages []Message
			err = json.Unmarshal(messagesFile, &messages)
			CheckError(err)

			totalMessagesCount += len(messages)

			for _, message := range messages {
				message.Channel = channel.Name
				if message.UserID == "" {
					if message.BotID == "" {
						continue
					} else {
						if userIDSet[message.BotID] {
							message.UserID = message.BotID
						} else {
							newBot := User{
								ID:      message.BotID,
								Name:    message.BotUsername,
								Deleted: false,
								IsBot:   true,
							}
							newBot.Profile.DisplayName = message.BotUsername
							newBot.Profile.RealName = message.BotUsername
							users = append(users, newBot)

							query := "INSERT INTO users (id, name, real_name, display_name, email, deleted, is_bot, image_url) VALUES ($1, $2, $3, $4, $5, $6, $7, $8);"
							_, err = db.Exec(query, newBot.ID, newBot.Name, newBot.Profile.RealName, newBot.Profile.DisplayName, newBot.Profile.Email, newBot.Deleted, newBot.IsBot, newBot.Profile.ImageURL)
							CheckError(err)
							userIDSet[newBot.ID] = true

							message.UserID = message.BotID
						}
					}
				}

				text := ""
				if message.SubType == "channel_join" {
					text = "<em>Joined the channel</em>"
				} else if message.SubType == "channel_archive" {
					text = "<em>Archived the channel</em>"
				} else if message.SubType == "channel_leave" {
					text = "<em>Left the channel</em>"
				} else if len(message.Blocks) > 0 {
					text = parseMessage(message.Blocks)
				}
				if text == "" {
					text = message.Text
				}

				if message.ThreadTimestamp != "" {
					query := "INSERT INTO messages (channel_name, user_id, ts, msg_text, parent_user_id, thread_ts) VALUES ($1, $2, TIMESTAMP 'epoch' + $3 * INTERVAL '1 second', $4, $5, TIMESTAMP 'epoch' + $6 * INTERVAL '1 second');"
					_, err = db.Exec(query, message.Channel, message.UserID, message.Timestamp, text, message.ParentUserID, message.ThreadTimestamp)
				} else {
					query := "INSERT INTO messages (channel_name, user_id, ts, msg_text, parent_user_id) VALUES ($1, $2, TIMESTAMP 'epoch' + $3 * INTERVAL '1 second', $4, $5);"
					_, err = db.Exec(query, message.Channel, message.UserID, message.Timestamp, text, message.ParentUserID)
				}
				CheckError(err)
				totalMessagesAddedCount++
			}
		}
	}
	log.Println("Digester found and sent " + fmt.Sprint(totalMessagesAddedCount) + "/" + fmt.Sprint(totalMessagesCount) + " messages to the tummy.")
}
