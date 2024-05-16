package main

import "fmt"

type Block struct {
	Type     string    `json:"type"`
	Elements []Element `json:"elements"`
}

type Element struct {
	Type      string    `json:"type"`
	Elements  []Element `json:"elements"`
	Text      string    `json:"text"`
	ListStyle string    `json:"style"`
	EmojiName string    `json:"name"`
}

func parseList(elements []Element) string {
	result := ""

	for _, element := range elements {
		result += "- " + parseElement(element) + "\n"
	}

	return result
}

func parseElement(element Element) string {
	result := ""

	switch element.Type {
	case "text":
		result = element.Text
	case "emoji":
		result = ":" + element.EmojiName + ":"
	case "rich_text_section":
		for _, subElement := range element.Elements {
			result += parseElement(subElement)
		}
	case "rich_text_list":
		result += parseList(element.Elements)
	default:
		fmt.Println("Unknown element type: " + element.Type)
	}

	return result
}

func parseBlock(block Block) string {
	result := ""

	for _, element := range block.Elements {
		result += parseElement(element)
	}

	return result
}

func parse(blocks []Block) string {
	result := ""

	for _, block := range blocks {
		result += parseBlock(block)
	}

	return result
}
