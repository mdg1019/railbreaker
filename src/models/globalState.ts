export class GlobalState {
  tracks: Map<string, string>;
  currentDirectory: string;
  downloadsDirectory: string;
  racecardsDirectory: string;

  constructor(
    tracks: Map<string, string> = new Map<string, string>(),
    currentDirectory: string = "",
    downloadsDirectory: string = "",
    racecardsDirectory: string = ""
  ) {
    this.tracks = tracks;
    this.currentDirectory = currentDirectory;
    this.downloadsDirectory = downloadsDirectory;
    this.racecardsDirectory = racecardsDirectory;
  }

  static fromObject(obj: any): GlobalState {
    const tracks = obj?.tracks
      ? new Map<string, string>(Object.entries(obj.tracks))
      : new Map<string, string>();
    return new GlobalState(
      tracks,
      obj?.currentDirectory || "",
      obj?.downloadsDirectory || "",
      obj?.racecardsDirectory || ""
    );
  }

  toObject(): any {
    return {
      tracks: Object.fromEntries(this.tracks),
      currentDirectory: this.currentDirectory,
      downloadsDirectory: this.downloadsDirectory,
      racecardsDirectory: this.racecardsDirectory,
    };
  }
}
