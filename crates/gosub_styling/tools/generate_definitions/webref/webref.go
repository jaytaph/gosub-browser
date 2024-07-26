package webref

import (
	"encoding/json"
	"generate_definitions/utils"
	"io"
	"log"
	"net/http"
	"os"
	"regexp"
	"strings"
	"sync"
)

var (
	skipList = []string{
		"css-borders",
	}
)

type Selector struct {
	Name string `json:"name"`
}

type Data struct {
	Spec       Spec               `json:"spec"`
	Properties []WebRefProperties `json:"properties"`
	Values     []WebRefValue      `json:"values"`
	AtRules    []WebRefAtRule     `json:"atrules"`
	Selectors  []Selector         `json:"selectors"`
}

type Spec struct {
	Title string `json:"title"`
	Url   string `json:"url"`
}

type WebRefValue struct {
	Name   string `json:"name"`
	Syntax string `json:"value"`
}

type WebRefProperties struct {
	Name      string   `json:"name"`
	Syntax    string   `json:"value"`
	NewSyntax string   `json:"newValues"`
	Computed  []string `json:"computed"`
	//Initial   StringMaybeArray `json:"initial"`
	Inherited string `json:"inherited"`
}

type WebRefAtRule struct {
	Name        string `json:"name"`
	Descriptors []struct {
		Name    string `json:"name"`
		Syntax  string `json:"value"`
		Initial string `json:"initial"`
	} `json:"descriptors"`
	Syntax string `json:"value"`
	Values []struct {
		Name   string `json:"name"`
		Value  string `json:"value,omitempty"`
		Values []struct {
			Name  string `json:"name"`
			Value string `json:"value"`
		}
	}
}

func GetWebRefFiles() []utils.DirectoryListItem {
	filesResp, err := http.Get("https://api.github.com/repos/" + utils.REPO + "/contents/" + utils.LOCATION)
	if err != nil {
		log.Fatal(err)
	}

	defer filesResp.Body.Close()

	body, err := io.ReadAll(filesResp.Body)
	if err != nil {
		log.Fatal(err)
	}

	var files []utils.DirectoryListItem
	if err := json.Unmarshal(body, &files); err != nil {
		log.Fatal(err)
	}

	return files
}

func GetWebRefData() Data {
	files := GetWebRefFiles()

	var data Data
	mu := new(sync.Mutex)

	wg := new(sync.WaitGroup)

	//s := specs.GetSpecifications()

	properties := make(map[string]WebRefProperties)
	values := make(map[string]WebRefValue)
	atRules := make(map[string]WebRefAtRule)
	selectors := make(map[string]Selector)

	for _, file := range files {
		if file.Type != "file" || !strings.HasSuffix(file.Name, ".json") {
			continue
		}

		wg.Add(1)
		go func() {
			defer wg.Done()
			shortname := strings.TrimSuffix(file.Name, ".json")

			if matched, err := regexp.Match(`\d+$`, []byte(shortname)); err != nil || matched {
				return
			}

			if !skip(shortname) {
				log.Println("Skipping non-W3C spec", shortname)
				return
			}

			content, err := GetFileContent(&file)
			if err != nil {
				log.Fatal(file.Path, " ", err)
			}

			var fileData Data
			if err := json.Unmarshal(content, &fileData); err != nil {
				log.Fatal(file.Path, " ", err)
			}

			mu.Lock()
			defer mu.Unlock()

			for _, property := range fileData.Properties {
				if p, ok := properties[property.Name]; ok {
					if p.Syntax == "" {
						p.Syntax = property.Syntax
					} else if p.Syntax != property.Syntax && property.Syntax != "" {
						log.Println("Different syntax for duplicated property", property.Name)
						log.Println("Old:", p.Syntax)
						log.Println("New:", property.Syntax)
						//log.Fatal("Syntax mismatch")
					}

					if p.NewSyntax != "" && p.Syntax != "" {
						p.Syntax += " | " + p.NewSyntax
						p.NewSyntax = ""
					}

					if property.NewSyntax != "" {
						if p.Syntax != "" {
							p.Syntax += " | " + property.NewSyntax
						} else if p.NewSyntax != "" {
							p.NewSyntax += " | " + property.NewSyntax
						} else {
							p.NewSyntax = property.NewSyntax
						}
					}

					properties[p.Name] = p
					continue
				}

				properties[property.Name] = property
			}

			for _, value := range fileData.Values {
				if v, ok := values[value.Name]; ok {
					if v.Syntax == "" {
						v.Syntax = value.Syntax
					}

					if v.Syntax != "" && value.Syntax != "" && v.Syntax != value.Syntax {
						log.Println("Different syntax for duplicated value", value.Name)
						log.Println("Old:", v.Syntax)
						log.Println("New:", value.Syntax)
						//log.Fatal("Syntax mismatch")
					}

					values[v.Name] = v
					continue
				}
				values[value.Name] = value
			}

			for _, atRule := range fileData.AtRules {
				if a, ok := atRules[atRule.Name]; ok {
					if a.Syntax == "" {
						a.Syntax = atRule.Syntax
					}

					if a.Syntax != "" && atRule.Syntax != "" && a.Syntax != atRule.Syntax {
						log.Println("Different syntax for duplicated at-rule", atRule.Name)
						log.Println("Old:", a.Syntax)
						log.Println("New:", atRule.Syntax)
						//log.Fatal("Syntax mismatch")
					}

					a.Values = append(a.Values, atRule.Values...)
					a.Descriptors = append(a.Descriptors, atRule.Descriptors...)

					atRules[a.Name] = a
					continue
				}
				atRules[atRule.Name] = atRule
			}

			for _, selector := range fileData.Selectors {
				selectors[selector.Name] = selector
			}

		}()
	}

	wg.Wait()

	return data
}

func GetFileContent(file *utils.DirectoryListItem) ([]byte, error) {
	cachePath := utils.CACHE_DIR + "/" + file.Name

	hash, err := utils.ComputeGitBlobSHA1(cachePath)
	if err != nil {
		log.Println("Failed to compute SHA1 for", file.Path)
	}

	if hash != file.Sha {
		log.Println("Cache file is outdated, downloading", file.Path)
		resp, err := http.Get(file.DownloadUrl)
		if err != nil {
			log.Fatal(file.Path, " ", err)
		}

		body, err := io.ReadAll(resp.Body)
		resp.Body.Close()
		if err != nil {
			return nil, err
		}

		if err := os.WriteFile(cachePath, body, 0644); err != nil {
			log.Println("Failed to write cache file", cachePath, err)
		}

		return body, nil
	}

	return os.ReadFile(cachePath)
}

func DetectDuplicates(data *Data) {
	properties := make(map[string]WebRefProperties)
	values := make(map[string]WebRefValue)
	atRules := make(map[string]WebRefAtRule)
	selectors := make(map[string]Selector)

	for _, property := range data.Properties {
		if p, ok := properties[property.Name]; ok {
			if p.Syntax != property.Syntax {
				log.Println("Different syntax for duplicated property", property.Name)
				log.Println("Old:", p.Syntax)
				log.Println("New:", property.Syntax)
			}
		}
		properties[property.Name] = property
	}

	for _, value := range data.Values {
		if _, ok := values[value.Name]; ok {
			log.Println("Duplicate value", value.Name)
		}
		values[value.Name] = value
	}

	for _, atRule := range data.AtRules {
		if _, ok := atRules[atRule.Name]; ok {
			log.Println("Duplicate at-rule", atRule.Name)
		}
		atRules[atRule.Name] = atRule
	}

	for _, selector := range data.Selectors {
		if _, ok := selectors[selector.Name]; ok {
			log.Println("Duplicate selector", selector.Name)
		}
		selectors[selector.Name] = selector
	}
}

func skip(shortname string) bool {
	for _, s := range skipList {
		if s == shortname {
			return false
		}
	}
	return true
}
