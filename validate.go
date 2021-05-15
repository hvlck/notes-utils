package main

import (
	"fmt"
	"io/fs"
	"log"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"unicode/utf8"
)

// Find finds all urls within a string
func validateNote(contents string) string {
	return regexp.MustCompile("# .+\r\n\r\n\x60.+\x60").FindString(contents)
}

// Validate validate all notes
func Validate() {
	err := filepath.Walk(".", func(file string, info fs.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() {
			return nil
		}

		i, _ := os.Stat(file)
		if strings.HasPrefix(file, ".") || i.IsDir() {
			return nil
		}

		f, err := os.ReadFile(file)
		if err != nil {
			log.Fatalf("failed to read file %s: %s", file, err)
		}

		if utf8.Valid(f) == false {
			fmt.Printf("%s is not valid utf-8, skipping...\n", file)
			return nil
		}

		if strings.HasSuffix(file, ".md") == true {
			matches := validateNote(string(f))
			if len(matches) == 0 {
				log.Fatalf("%s is invalid, exiting...", file)
			}
		}

		return nil
	})

	if err != nil {
		fmt.Printf("error reading directories: %v\n", err)
		return
	} else {
		fmt.Println("no invalid files!")
	}
}
