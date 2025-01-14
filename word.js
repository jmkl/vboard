const fs = require("fs");
const path = require("path");

function combine() {
  const inputFolder = "./words"; // Replace with the path to your folder containing text files
  const outputFile = "./combined.txt"; // Path for the output file

  // Function to combine all text files into one
  function combineTextFiles(folder, output) {
    try {
      const files = fs.readdirSync(folder);
      let combinedContent = "";

      files.forEach((file) => {
        const filePath = path.join(folder, file);

        // Check if the file is a text file
        if (fs.statSync(filePath).isFile() && path.extname(file) === ".txt") {
          const content = fs.readFileSync(filePath, "utf8");
          combinedContent += content + "\n"; // Add content and a newline
        }
      });

      fs.writeFileSync(output, combinedContent, "utf8");
      console.log(`All files combined into ${output}`);
    } catch (err) {
      console.error("Error combining files:", err.message);
    }
  }

  // Execute the function
  combineTextFiles(inputFolder, outputFile);
}

function processFile() {
  const input = "combined.txt";
  const output = "combined_trim.txt";
  try {
    // Read the file
    const fileContent = fs.readFileSync(input, "utf8");

    // Split into lines and remove duplicates
    const uniqueLines = [
      ...new Set(fileContent.split("\n").map((line) => line.trim())),
    ];

    // Filter out empty lines
    const filteredLines = uniqueLines.filter((line) => line !== "");

    // Write the output to a new file
    fs.writeFileSync(output, filteredLines.join("\n"), "utf8");

    console.log(`Processed content written to ${output}`);
  } catch (err) {
    console.error("Error processing file:", err.message);
  }
}
function dicMeUp() {
  const fileContent = fs.readFileSync("./words/index.dic", "utf8");
  const uniqueLines = fileContent
    .split("\n")
    .map((line) => line.trim().split("/")[0]);
  const filteredLines = uniqueLines.filter((line) => line !== "").join("\n");
  fs.writeFileSync("outputwords.txt", filteredLines, "utf8");

  console.log(uniqueLines);
}
processFile();
