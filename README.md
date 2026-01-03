# RailBreaker

Is a Rust/Tauri app for handicapping horse races. It uses the single-file data files from Brisnet.

The app can unzip and store a JSON version of the data in the Racecards directory. The unzipped *.DRF file is deleted when the JSON file is created. JSON allows the program to load the data faster rather than reprocessing the *.DRF file each time.
