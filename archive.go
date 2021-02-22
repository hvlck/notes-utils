// code modified from preen

package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"regexp"
	"strings"
	"unicode/utf8"
)

// Read utility function to get all files in a directory
func Read(dir string) []string {
	file, err := os.Open(dir)
	if err != nil {
		log.Fatalf("failed to read directory: %s\n", err)
	}

	defer file.Close()

	files, err := file.Readdirnames(0)
	if err != nil {
		log.Fatalf("failed to read contents of directory: %s\n", err)
	}

	return files
}

// Find finds all urls within a string
func Find(contents string) []string {
	return regexp.MustCompile(`https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)`).FindAllString(contents, -1)
}

// Save saves a file
func Save(body string, match string) {
	match = regexp.MustCompile(`https?:\/\/(www\.)?`).ReplaceAllString(match, "")
	match = strings.Join(strings.Split(match, "?")[:1], "")

	p := strings.Split(match, "/")
	path := strings.Join(p[0:len(p)-1], "/") + "/"

	e := strings.Split(match, ".")
	ext := e[len(e)-1]
	if regexp.MustCompile(".(.css|.js|.png|.jpeg)$").MatchString(match) == false {
		ext = "/index.html" // fallback for naked routes (e.g. /test/route)
	}

	_, err := os.Stat("./archive" + path)
	if os.IsNotExist(err) && len(path) > 1 {
		os.MkdirAll("./archive/"+path, 0755)
	}

	final := path + ext
	if ext != "/index.html" {
		final = path + strings.ReplaceAll(match, path, "")
	}
	f, err := os.Create("archive/" + final)
	if err != nil {
		log.Fatalf("failed to create file %s: %s", match, err)
	}

	_, err = f.WriteString(body)
	if err != nil {
		log.Fatalf("failed to write contents to file %s: %s", match, err)
	}
}

func download(match string) {
	if _, pres := downloaded[match]; pres == true {
		return
	}

	r, err := http.Get(match)

	if err != nil {
		log.Printf("failed to get %s: %s\n", match, err)
	}

	if r.StatusCode != 200 {
		log.Printf("%s responded with something other than status 200\n", match)
		fails++
	} else if r.Request.URL.String() != match {
		log.Printf("%s was redirected", match)
		fails++
	} else {
		fmt.Printf("successfully retrieved %s\n", match)

		body, err := ioutil.ReadAll(r.Body)
		if err != nil {
			log.Fatalf("failed to read body: %s", err)
		}

		Save(string(body), match)

		matches := Find(string(body))

		for _, m := range matches {
			if v, pres := downloaded[m]; pres != true && v != true {
				download(m)
				downloaded[m] = true
			}
		}
	}

}

var fails uint32

var downloaded map[string]bool

// Archive archive all links in corpus
func Archive() {
	downloaded = make(map[string]bool)
	files := Read(".")

	_, err := os.Stat("./archive")
	if os.IsNotExist(err) {
		os.Mkdir("./archive", 0755)
	}

	for _, file := range files {
		i, _ := os.Stat(file)
		if strings.HasPrefix(file, ".") || i.IsDir() {
			continue
		}

		fmt.Printf("checking file %s\n", file)

		f, err := os.ReadFile(file)
		if err != nil {
			log.Fatalf("failed to read file %s: %s", file, err)
		}

		if utf8.Valid(f) == false {
			fmt.Printf("%s is not valid utf-8, skipping...\n", file)
			continue
		}

		matches := Find(string(f))

		for _, match := range matches {
			if strings.HasSuffix(file, ".md") == true {
				match = strings.TrimSuffix(match, ")") // removes excess parantheses from markdown links
			}

			if v, pres := downloaded[match]; pres != true && v != true {
				download(match)
				downloaded[match] = true
			}
		}

		fmt.Printf("finished checking file %s\n", file)
	}

	fmt.Printf("finished archiving all links, %v failed\n", fails)
}
