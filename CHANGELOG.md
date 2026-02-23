## 0.5.0 - First Beta Release
- I think the app is relatively stable and won't need many more new features right away. Therefore, I'm going to call this the first beta release. 
- Removed the sliding "Races" menu and replaced it with a menu bar that appears at the top of the window once a racecard has been opened. You can select an open racecard or a race from two dropdowns. There is also a "Sort" menu for selecting the current sort type.
- Switched to an NSIS installer for Windows. This installs the program in your user account folder, which avoids the need for a signing certificate. Yay!!!
## 0.4.2
- The trip handicapper no longer filters out trips older than 60 days. The number of days is highlighted in purple to show the significance of the date. The trip handicapper is really great for looking at trip information. So showing older trips is beneficial to the user.
## 0.4.1
- Fixed the issue with Windows not recognizing hotkeys.
## 0.4.0
- RailBreaker now requires using the railbreaker-lib library to compile. See the README.md for details.
- Added a Trip Handicapping Model. This is a work in progress. Check out the tooltips over the trip info. Super handy.
- Lots of UI changes. Added support for the Trip Handicapping Model. The horses can be sorted (see View -> Sort Horses menu option). Hotkeys are available for Next and Previous races. 
- Added a Help dialog. It explains the available hotkeys and has warnings about the handicapping model. Use the models as guides!!!
- I'm using a Pinia state to better handle the racecard info.
## 0.3.2
- Improved the code that was allowing an incorrectly formatted Brisnet file load.
- Added an About dialog box.
## 0.3.1
- Fixed a problem due to an incorrectly formatted Brisnet file. One line had an extra column, which was blowing up the parser. So I just chop off the ***Reserved*** fields at the end of the line.
- Refactored how scratches are done. The old code worked, but it was kind of sloppy.
## 0.3.0
- Added a Contextual Speed and Pace Model to try to project the outcome of a race. Use any results with caution. If a computer was really good at picking winning horse races, the tracks would be out of business a long time ago. It's a guide. Nothing more. Because the model is dependent upon scratches, it only appears in the display screen and not in printouts. There are many factors that can skew a race, track bias, jockey change, trainer change, bad weather, etc.
## 0.2.0
- Added notes. You can create a note for each horse. Empty notes are ignored in printouts. 
- Racecards are now saved in a Sqlite database in the Racecards directory with the filenames "railbreaker.db*"
- Added some new UI elements to support the changes for Sqlite and added more consistent coloring to the dialog boxes.
- NOTE: If you've used a version previous to 0.2.0, you can delete any *.json files in the Racecards directory.
## 0.1.2 
- The app will no longer unzip or load a racecard that's already in memory.
- I've reduced the line height for printouts to conserve on paper.