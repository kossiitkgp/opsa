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

	"github.com/joho/godotenv"
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
	Channel      string
	UserID       string `json:"user"`
	TS           string `json:"ts"`
	Text         string `json:"text"`
	ThreadTS     string `json:"thread_ts"`
	ParentUserID string `json:"parent_user_id"`
}

const (
	ZIPFILE_PATH      = "/slack-export.zip"
	EXTRACTION_DIR    = "/extracted"
	USERS_FILENAME    = "users.json"
	CHANNELS_FILENAME = "channels.json"
)

var db *sql.DB

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
	err := godotenv.Load(".env")
	if err != nil {
		log.Println("WARNING: " + err.Error())
	}

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

	usersFile, err := os.ReadFile(EXTRACTION_DIR + string(os.PathSeparator) + USERS_FILENAME)
	CheckError(err)
	var users []User
	err = json.Unmarshal(usersFile, &users)
	CheckError(err)
	log.Println("Digester found " + fmt.Sprint(len(users)) + " users.")

	channelsFile, err := os.ReadFile(EXTRACTION_DIR + string(os.PathSeparator) + CHANNELS_FILENAME)
	CheckError(err)
	var channels []User
	err = json.Unmarshal(channelsFile, &channels)
	CheckError(err)
	log.Println("DIgester found " + fmt.Sprint(len(channels)) + " channels.")
}
