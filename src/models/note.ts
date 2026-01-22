export class Note {
  race: number;
  horse: number;
  content: string;

  constructor(race: number = 0, horse: number = 0, content: string = "") {
    this.race = race;
    this.horse = horse;
    this.content = content;
  }

  static fromObject(obj: any): Note {
    return new Note(obj?.race ?? 0, obj?.horse ?? 0, obj?.content ?? "");
  }

  toObject(): any {
    return {
      race: this.race,
      horse: this.horse,
      content: this.content,
    };
  }
}
