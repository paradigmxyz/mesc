package main

import (
	"context"
	"flag"
	"fmt"
	"io"
	"os"

	"github.com/paradigmxyz/mesc/go/pkg/mesc"
	"github.com/paradigmxyz/mesc/go/pkg/mesc/endpoint/io/serialization"
)

func main() {
	ctx := context.Background()

	var queryType string
	flag.StringVar(&queryType, "query-type", "", "the type of query to execute")

	flag.Parse()

	switch queryType {
	case "get_default_endpoint":
		endpoint, err := mesc.GetDefaultEndpoint(ctx)
		if err != nil {
			printFailure(fmt.Errorf("failed to get default endpoint: %w", err))
			return
		}

		jsonModel, err := serialization.SerializeEndpointMetadataJSON(endpoint)
		if err != nil {
			printFailure(fmt.Errorf("failed to serialize endpoint metadata to JSON: %w", err))
			return
		}

		_, _ = io.Copy(os.Stdout, jsonModel)

		return
	}
}

func printFailure(e error) {
	fmt.Printf("FAIL: %v\n", e)
}
