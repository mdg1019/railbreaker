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
    const data = toCamelCaseKeys(obj ?? {});
    const tracks = data.tracks
      ? new Map<string, string>(Object.entries(data.tracks))
      : new Map<string, string>();
    return new GlobalState(
      tracks,
      data.currentDirectory || "",
      data.downloadsDirectory || "",
      data.racecardsDirectory || ""
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

const toCamelCaseKey = (key: string): string =>
  key.replace(/_([a-z0-9])/g, (_, letter) => letter.toUpperCase());

const toCamelCaseKeys = (value: any): any => {
  if (Array.isArray(value)) {
    return value.map(toCamelCaseKeys);
  }

  if (value && typeof value === 'object' && value.constructor === Object) {
    const result: Record<string, any> = {};
    for (const [key, val] of Object.entries(value)) {
      result[toCamelCaseKey(key)] = toCamelCaseKeys(val);
    }
    return result;
  }

  return value;
};
