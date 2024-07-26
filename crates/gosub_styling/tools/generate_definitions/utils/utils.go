package utils

import (
	"crypto/sha1"
	"fmt"
	"os"
)

const (
	REPO      = "w3c/webref"
	LOCATION  = "ed/css"
	CACHE_DIR = "crates/gosub_styling/resources/cache"
)

func ComputeGitBlobSHA1(filePath string) (string, error) {
	content, err := os.ReadFile(filePath)
	if err != nil {
		return "", err
	}

	return ComputeGitBlobSHA1Content(content), nil
}

func ComputeGitBlobSHA1Content(content []byte) string {

	header := fmt.Sprintf("blob %d\000", len(content))

	data := append([]byte(header), content...)

	hash := sha1.Sum(data)

	return fmt.Sprintf("%x", hash)
}

type DirectoryListItem struct {
	Name        string `json:"name"`
	Path        string `json:"path"`
	Sha         string `json:"sha"`
	Size        int    `json:"size"`
	DownloadUrl string `json:"download_url"`
	Type        string `json:"type"`
}
