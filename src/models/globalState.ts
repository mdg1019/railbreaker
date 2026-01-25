export class GlobalState {
  currentDirectory: string;
  downloadsDirectory: string;
  racecardsDirectory: string;

  constructor(
    currentDirectory: string = "",
    downloadsDirectory: string = "",
    racecardsDirectory: string = ""
  ) {
    this.currentDirectory = currentDirectory;
    this.downloadsDirectory = downloadsDirectory;
    this.racecardsDirectory = racecardsDirectory;
  }

  static fromObject(obj: any): GlobalState {
    return new GlobalState(
      obj?.currentDirectory || "",
      obj?.downloadsDirectory || "",
      obj?.racecardsDirectory || ""
    );
  }

  toObject(): any {
    return {
      currentDirectory: this.currentDirectory,
      downloadsDirectory: this.downloadsDirectory,
      racecardsDirectory: this.racecardsDirectory,
    };
  }
}
