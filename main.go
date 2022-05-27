// go-thumbnailer: A cover thumbnailer written in Go for performance and reliability.
// Copyright (C) 2022 Donovan Glover

package main

import (
  "io/ioutil"
  "os"
  "strconv"
  "github.com/davidbyttow/govips/v2/vips"
)

func main() {
  if len(os.Args) < 4 {
    os.Exit(1)
  }

  size, sizeErr := strconv.Atoi(os.Args[1])

  if (sizeErr != nil) {
    size = 128
  }

  inputDirectory := os.Args[2]
  outputFile := os.Args[3]

  // Make the thumbnail
  vips.Startup(nil)
  thumbnail := getThumbnail(inputDirectory, size)

  // Export as png
  png := vips.NewDefaultPNGExportParams()
  pngBytes, _, errImg := thumbnail.Export(png)

  // Save to outputFile
  errImg = ioutil.WriteFile(outputFile, pngBytes, 0644)

  if errImg != nil {
    os.Exit(3)
  }
}

func getThumbnail(inputFolder string, size int) (thumbnail *vips.ImageRef) {
  jpg, errJPG := vips.NewThumbnailFromFile(inputFolder + "/cover.jpg", size, size, vips.InterestingCentre)

  if errJPG != nil {
    png, errPNG := vips.NewThumbnailFromFile(inputFolder + "/cover.png", size, size, vips.InterestingCentre)

    if errPNG != nil {
      os.Exit(2)
    } else {
      thumbnail = png
    }

  } else {
    thumbnail = jpg
  }

  return
}
