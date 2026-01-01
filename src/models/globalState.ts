export class GlobalState {
  tracks: Map<string, string>;
  current_directory: string;
  downloads_directory: string;
  racecards_directory: string;

  constructor(
    tracks: Map<string, string> = new Map<string, string>(),
    current_directory: string = "",
    downloads_directory: string = "",
    racecards_directory: string = ""
  ) {
    this.tracks = tracks;
    this.current_directory = current_directory;
    this.downloads_directory = downloads_directory;
    this.racecards_directory = racecards_directory;
  }

  static fromObject(obj: any): GlobalState {
    const tracks = obj.tracks 
      ? new Map<string, string>(Object.entries(obj.tracks))
      : new Map<string, string>();
    return new GlobalState(
      tracks,
      obj.current_directory || "",
      obj.downloads_directory || "",
      obj.racecards_directory || ""
    );
  }

  toObject(): any {
    return {
      tracks: Object.fromEntries(this.tracks),
      current_directory: this.current_directory,
      downloads_directory: this.downloads_directory,
      racecards_directory: this.racecards_directory,
    };
  }
}
