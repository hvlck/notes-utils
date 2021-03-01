package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"os/exec"
	"strings"
)

func main() {
	check := flag.NewFlagSet("check", flag.ExitOnError)

	new := flag.NewFlagSet("new", flag.ExitOnError)
	newName := new.String("name", "", "name of new note")

	backlink := flag.NewFlagSet("backlink", flag.ExitOnError)

	archive := flag.NewFlagSet("archive", flag.ExitOnError)

	validate := flag.NewFlagSet("validate", flag.ExitOnError)

	if len(os.Args) < 2 {
		fmt.Println("no subcommand invoked\nrun notes_utils help to view usage")
		os.Exit(1)
	}

	switch os.Args[1] {
	case "check":
		check.Parse(os.Args[2:])

		bin, err := exec.LookPath("preen")
		if err != nil {
			fmt.Printf("failed to find executable: %s\n", err)
		}

		cmd := exec.Command("preen", "./")
		cmd.Path = bin

		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
		cmd.Stderr = os.Stderr

		res := cmd.Run()
		if res != nil {
			log.Fatalf("failed to run preen: %s", res)
		}
	case "new":
		new.Parse(os.Args[2:])

		fn := strings.ReplaceAll(strings.ToLower(*newName), " ", "-")
		f, err := os.Create(fn + ".md")
		if err != nil {
			log.Fatalf("failed to create file %s: %s", *newName, err)
		}
		_, err = f.WriteString("# " + *newName + "\n\n## See Also\n")

		if err != nil {
			log.Fatalf("failed to write to file %s: %s", *newName, err)
		} else {
			fmt.Printf("successfully created note %s", *newName)
		}
	case "backlink":
		backlink.Parse(os.Args[2:])
	case "archive":
		archive.Parse(os.Args[2:])
		Archive()
	case "validate":
		validate.Parse(os.Args[2:])
		Validate()
	default:
		fmt.Println("invalid subcommand invoked")
		os.Exit(1)
	}
}
