# RailBreaker

Is a Rust/Tauri app for handicapping horse races. It uses the single-file data files from Brisnet.

#### Railbreaker can do this at present:

- Unzip and load a racecard file.
- Load a previously unzipped racecard file.
- Load multiple racecards. (Racecard selection is changed in the sliding-left menu.)
- The basic racecard is complete. (I need to proofread it more against Brisnet PPs.)
- Added a tooltip to the comments. When clicking on a comment, you can see the extended comment if one is available. I'll be adding more helpful tooltips like this where I think they'd be useful.
- Printouts are working. It prints the whole card right now, but I plan to add a dialog box to allow the user to select either all races or select which ones to print.
- Added a Print Dialog box so that the user can select what races should be printed. See pic below.
- Switched to a Sqlite database instead of the .json files.
- Notes for each horse are working.
- Contextual Speed and Pace Model attempts to determine the outcome of each race.

#### Things to do:

- Add more bells and whistles over time to the UI.
- Build a companion program that will take the Contextual Speed and Pace Model and compare that with data from the PP and a Brisnet comprehensive results file to see where the model did well and where it didn't do well. Hopefully, this can spot patterns to improve the analysis. This will take time to build the data to see if anything pops out.
- Research other handicapping models.

#### Windows Users:
I do not have a signing certificate. You will be asked about this during installation. Go ahead and install it. To run the app, you need to run it as an Administrator. Just right-click the app's icon and select ***Run as Administrator***.

#### Racecard is Complete:

![racecard](images/racecard.png)

#### Sliding Menu

![sliding menu](images/sliding-menu.png)

#### Sample Printout

![sample printout](images/printed-form.png)

#### Print Dialog Box

![print dialog box](images/print-dialog.png)

#### Contextual Speed and Pace Model
![contextual speed and pace model](images/cspm.png)