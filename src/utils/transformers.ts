import type { Race } from '../models/racecard'
// function match<T, R>(value: T, cases: Array<[(v: T)=>boolean, (v: T)=>R]>, otherwise: ()=>R): R {
//   for (const [pred, fn] of cases) if (pred(value)) return fn(value);
//   return otherwise();
// }

export default class Transformers {
  static getRaceClassification(race: Race): [string, string] {
    if (!race) return ['', ''];

    let raceClassification = race.todays_race_classification;

    if (race.race_type == "M" && raceClassification.startsWith('Md ')) {
      raceClassification = 'MC ' + raceClassification.slice(3)
    }

    let prefix = "";

    if (race.age_sex_restrictions[2].toLowerCase() == "f") {
      prefix = '\u00EA';
    }

    if (race.statebred_flag.toLowerCase() == "s") {
      prefix += '\u00EB';
    }

    return [prefix, raceClassification];  }
}
