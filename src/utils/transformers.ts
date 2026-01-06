import type { Race } from "../models/racecard";

// function match<T, R>(value: T, cases: Array<[(v: T)=>boolean, (v: T)=>R]>, otherwise: ()=>R): R {
//   for (const [pred, fn] of cases) if (pred(value)) return fn(value);
//   return otherwise();
// }

export default class Transformers {
    static getRaceClassification(race: Race): [string, string] {
        if (!race) return ["", ""];

        let raceClassification = race.todays_race_classification;

        if (race.race_type == "M" && raceClassification.startsWith("Md ")) {
            raceClassification = "MC " + raceClassification.slice(3);
        }

        let prefix = "";

        if (race.age_sex_restrictions[2].toLowerCase() == "f") {
            prefix += "\u00EA";
        }

        if (race.statebred_flag.toLowerCase() == "s") {
            prefix += "\u00EB";
        }

        return [prefix, raceClassification];
    }

    static getDistanceString(distance: number): string {
        const FRACTION_UNICODE_MAP: { [k: number]: string } = {
            0.0625: "\u00C0", // 1/16
            0.125:  "\u00C1", // 2/16
            0.1875: "\u00C2", // 3/16
            0.25:   "\u00C3", // 4/16
            0.3125: "\u00C4", // 5/16
            0.375:  "\u00C5", // 6/16
            0.4375: "\u00C6", // 7/16
            0.5:    "\u00C7", // 8/16
            0.5625: "\u00C8", // 9/16
            0.625:  "\u00C9", // 10/16
            0.6875: "\u00CA", // 11/16
            0.75:   "\u00CB", // 12/16
            0.8125: "\u00CC", // 13/16
            0.875:  "\u00CD", // 14/16
            0.9375: "\u00CE"  // 15/16
        };

        let isAbout = false;
        let isMiles = false;

        if (distance < 0) {
            isAbout = true;
            distance = -distance;
        }

        let value = 0;

        if (distance >= 1760) {
            isMiles = true;
            value = distance / 1760;
        } else {
            value = distance / 220;
        }

        let whole = Math.floor(value);
        let rem = value - whole;

        let fracStr = FRACTION_UNICODE_MAP[rem] || "";
        
        let result = String(whole) + fracStr;

        if (isAbout) {
            result = "~" + result;
        }

        return result;
    }

    static getDistanceLength(distance: number): string {
        let result = this.getDistanceString(distance);

        if (distance === 1760) {
            return result + " Mile";
        }

        if (distance > 1760) {
            return result + " Miles";
        } 
        
        return result + " Furlongs";
    }

    static getAgeSexRestrictionString(restriction: string): string {
        const AGE_MAP: { [k: string]: string } = {
            "A": "2",
            "B": "3",
            "C": "4",
            "D": "5",
            "E": "3&4",
            "F": "4&5",
            "G": "3,4&5",
            "H": "All Ages"  
        };

        const SEX_MAP: { [k: string]: string } = {
            "M": "Mares & Fillies",
            "C": "Colts & Geldings",
            "F": "Fillies",
        };

        console.log("Restriction:", restriction);

        let result = AGE_MAP[restriction[0]];

        result += restriction[1] == "U" ? "&up" : "yo";

        if (restriction[2] == "N") return result;

        return result + " " + SEX_MAP[restriction[2]];
    }

    static capitalize(s: string): string {
        if (!s) return "";
        
        return s.toLowerCase().replace(/\w\S*/g, (txt) => {
            return txt.charAt(0).toUpperCase() + txt.slice(1);
        });
    }
    
}


