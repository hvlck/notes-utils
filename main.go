package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"os/exec"
)

func main() {
	check := flag.NewFlagSet("check", flag.ExitOnError)

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
	default:
		fmt.Println("invalid subcommand invoked")
		os.Exit(1)
	}
}
