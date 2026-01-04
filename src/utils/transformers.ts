import type { Race } from '../models/racecard'
function match<T, R>(value: T, cases: Array<[(v: T)=>boolean, (v: T)=>R]>, otherwise: ()=>R): R {
  for (const [pred, fn] of cases) if (pred(value)) return fn(value);
  return otherwise();
}

export default class Transformers {
  static getRaceType(race: Race): string {
    if (!race) return ''

    const raceType = race.todays_race_classification ?? race.race_type ?? ''
    const val = typeof raceType === 'string' ? raceType : String(raceType)

    const hasClaiming = race && (race.claiming_price != null && String(race.claiming_price).trim() !== '')
    if (hasClaiming && val.startsWith('Md ')) {
      return 'MC ' + val.slice(3)
    }

    return val
  }
}
