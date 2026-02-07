export class ConfigState {
  public lastDirectory: string;
  public windowX: number | null;
  public windowY: number | null;
  public windowWidth: number | null;
  public windowHeight: number | null;
  public horseSortingMethod: string;

  constructor(lastDirectory: string = '') {
    this.lastDirectory = lastDirectory;
    this.windowX = null;
    this.windowY = null;
    this.windowWidth = null;
    this.windowHeight = null;
    this.horseSortingMethod = 'program-number';
  }

  static fromObject(obj: any): ConfigState {
    const cs = new ConfigState(obj.last_directory || obj.lastDirectory || '');
    cs.windowX = obj.window_x ?? obj.windowX ?? null;
    cs.windowY = obj.window_y ?? obj.windowY ?? null;
    cs.windowWidth = obj.window_width ?? obj.windowWidth ?? null;
    cs.windowHeight = obj.window_height ?? obj.windowHeight ?? null;
    cs.horseSortingMethod =
      obj.horse_sorting_method ?? obj.horseSortingMethod ?? 'program-number';
    return cs;
  }

  toObject(): any {
    return {
      last_directory: this.lastDirectory,
      window_x: this.windowX,
      window_y: this.windowY,
      window_width: this.windowWidth,
      window_height: this.windowHeight,
      horse_sorting_method: this.horseSortingMethod,
    };
  }
}
