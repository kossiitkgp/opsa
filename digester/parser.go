package main

import (
	"encoding/json"
	"fmt"
	"strings"

	"github.com/enescakir/emoji"
	"github.com/gomarkdown/markdown"
	"github.com/gomarkdown/markdown/html"
	"github.com/gomarkdown/markdown/parser"
	"github.com/rs/zerolog/log"
)

type Block struct {
	Type     string    `json:"type"`
	Elements []Element `json:"elements"`
}

type Element struct {
	Type           string    `json:"type"`
	Elements       []Element `json:"elements"`
	Text           string    `json:"text"`
	Style          Style     `json:"style"`
	Indent         int       `json:"indent"`
	Border         int       `json:"border"`
	EmojiName      string    `json:"name"`
	URL            string    `json:"url"`
	UserID         string    `json:"user_id"`
	ChannelID      string    `json:"channel_id"`
	BroadcastRange string    `json:"range"`
	ColorValue     string    `json:"value"`
}

type Style struct {
	IsList    bool
	ListStyle string
	TextStyle TextStyle
}

type TextStyle struct {
	Bold   bool `json:"bold"`
	Italic bool `json:"italic"`
	Strike bool `json:"strike"`
	Code   bool `json:"code"`
}

func (s *Style) UnmarshalJSON(data []byte) error {
	var listStyle string
	if err := json.Unmarshal(data, &listStyle); err == nil {
		s.IsList = true
		s.ListStyle = listStyle
		return nil
	}

	var textStyle TextStyle
	if err := json.Unmarshal(data, &textStyle); err == nil {
		s.IsList = false
		s.TextStyle = textStyle
		return nil
	}

	return fmt.Errorf("unknown style type")
}

func addBorder(text string, border int) string {
	result := strings.Repeat(">", border)

	if border != 0 {
		result += " "
	}

	return result + text
}

func addIndent(text string, indent int) string {
	return strings.Repeat("   ", indent) + text
}

func parseText(element Element) string {
	result := ""

	if element.Type != "text" {
		log.Warn().Msg("Element is not text")
		return result
	}

	if element.Style.IsList {
		log.Warn().Msg("List element tried to be parsed as text")
		return result
	}

	leadingSpacesCount := 0
	for _, char := range element.Text {
		if char == ' ' {
			leadingSpacesCount++
		} else {
			break
		}
	}

	if leadingSpacesCount == len(element.Text) {
		return element.Text
	}

	trailingSpacesCount := 0
	for i := len(element.Text) - 1; i >= 0; i-- {
		if element.Text[i] == ' ' {
			trailingSpacesCount++
		} else {
			break
		}
	}

	result = element.Text[leadingSpacesCount : len(element.Text)-trailingSpacesCount]

	if element.Style.TextStyle.Bold {
		result = "**" + result + "**"
	}
	if element.Style.TextStyle.Italic {
		result = "*" + result + "*"
	}
	if element.Style.TextStyle.Strike {
		result = "~~" + result + "~~"
	}
	if element.Style.TextStyle.Code {
		result = "`" + result + "`"
	}

	return strings.Repeat(" ", leadingSpacesCount) + result + strings.Repeat(" ", trailingSpacesCount)
}

func parseList(element Element) string {
	result := "\n"

	if !element.Style.IsList {
		log.Warn().Msg("Element is not list")
		return result
	}

	if element.Style.ListStyle == "ordered" {
		for index, subElement := range element.Elements {
			result += addBorder(addIndent(fmt.Sprint((index+1))+". "+parseElement(subElement)+"\n", element.Indent), element.Border)
			if element.Border != 0 {
				result += "\n"
			}
		}
	} else {
		for _, subElement := range element.Elements {
			result += addBorder(addIndent("- "+parseElement(subElement)+"\n", element.Indent), element.Border)
			if element.Border != 0 {
				result += "\n"
			}
		}
	}

	return result
}

func parseQuote(element Element) string {
	result := ""

	for _, subElement := range element.Elements {
		result += parseElement(subElement)
	}
	result = strings.ReplaceAll(result, "\n", "\n> ")

	return addBorder(result, 1) + "\n\n"
}

func parsePreformatted(element Element) string {
	result := "```\n"
	for _, subElement := range element.Elements {
		result += parseElement(subElement)
	}
	result += "\n```"

	return result
}

func parseUser(element Element) string {
	result := "@"
	if _, userExists := userSet[element.UserID]; userExists {
		result += userSet[element.UserID]
	} else {
		result += "unknown-user"
	}
	return "<span class=\"user-mention\">" + result + "</span>"
}

func parseChannel(element Element) string {
	result := "#"
	if element.ChannelID == "" {
		result += "unknown-channel"
	} else if _, channelExists := channelSet[element.ChannelID]; channelExists {
		result += channelSet[element.ChannelID]
	} else {
		result += element.ChannelID
	}
	return "<span class=\"channel-mention\">" + result + "</span>"
}

func parseBroadcast(element Element) string {
	result := "@" + element.BroadcastRange
	if result == "@" {
		result += "unknown-broadcast"
	}
	return "<span class=\"broadcast-mention\">" + result + "</span>"
}

func parseLink(element Element) string {
	if element.Text == "" {
		element.Text = element.URL
	}

	return "[" + element.Text + "](" + element.URL + ")"
}

func parseSection(element Element) string {
	result := ""

	for _, subElement := range element.Elements {
		result += parseElement(subElement)
	}

	return result
}

func parseEmoji(element Element) string {
	return emoji.Parse(":" + element.EmojiName + ":")
}

func parseColor(element Element) string {
	return element.ColorValue
}

func parseElement(element Element) string {
	result := ""

	switch element.Type {
	case "text":
		result = parseText(element)
	case "emoji":
		result = parseEmoji(element)
	case "user":
		result = parseUser(element)
	case "channel":
		result = parseChannel(element)
	case "broadcast":
		result = parseBroadcast(element)
	case "color":
		result = parseColor(element)
	case "link":
		result = parseLink(element)
	case "rich_text_section":
		result = parseSection(element)
	case "rich_text_list":
		result += parseList(element)
	case "rich_text_quote":
		result = parseQuote(element)
	case "rich_text_preformatted":
		result = parsePreformatted(element)
	default:
		log.Warn().Msg("Unknown element type: " + element.Type + " (skipping)")
	}

	return result
}

func parseBlock(block Block) string {
	result := ""

	if block.Type != "rich_text" {
		log.Warn().Msg("Block is of unknown type: " + block.Type + " (skipping)")
		return result
	}

	for _, element := range block.Elements {
		result += parseElement(element)
	}

	return result
}

func parseMessage(blocks []Block) string {
	result := ""

	for _, block := range blocks {
		result += parseBlock(block) + "\n\n"
	}

	extensions := parser.CommonExtensions
	parser := parser.NewWithExtensions(extensions)
	doc := parser.Parse([]byte(strings.TrimSpace(result)))

	htmlFlags := html.CommonFlags | html.HrefTargetBlank
	opts := html.RendererOptions{Flags: htmlFlags}
	renderer := html.NewRenderer(opts)

	return string(markdown.Render(doc, renderer))
}
