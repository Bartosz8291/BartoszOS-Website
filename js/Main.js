        function downloadFile() {
            // URL of the file to download
            var url = "https://github.com/Bartosz8291/BartoszOS-Source-Code/archive/refs/heads/main.zip";

            // Create a temporary anchor element
            var anchor = document.createElement("a");
            anchor.href = url;
            anchor.download = "BartoszOS_Source_Code.zip";
            
            // Trigger the click event of the anchor element
            anchor.click();
        }
