import type { Race, PastPerformance } from "../models/racecard";

export default class Transformers {
    static TITLE_EXCEPTIONS = new Set([
        "and",
        "or",
        "the",
        "a",
        "an",
        "in",
        "on",
        "at",
        "of",
        "to",
        "for",
        "by",
        "with",
        "from",
        "vs",
        "v",
    ]);
    static MONTH_ABBREVIATIONS: string[] = [
        "Jan",
        "Feb",
        "Mar",
        "Apr",
        "May",
        "Jun",
        "Jul",
        "Aug",
        "Sep",
        "Oct",
        "Nov",
        "Dec",
    ];
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
            0.125: "\u00C1", // 2/16
            0.1875: "\u00C2", // 3/16
            0.25: "\u00C3", // 4/16
            0.3125: "\u00C4", // 5/16
            0.375: "\u00C5", // 6/16
            0.4375: "\u00C6", // 7/16
            0.5: "\u00C7", // 8/16
            0.5625: "\u00C8", // 9/16
            0.625: "\u00C9", // 10/16
            0.6875: "\u00CA", // 11/16
            0.75: "\u00CB", // 12/16
            0.8125: "\u00CC", // 13/16
            0.875: "\u00CD", // 14/16
            0.9375: "\u00CE", // 15/16
        };

        if (distance < 0) {
            distance = -distance;
        }

        let value = 0;

        switch (distance) {
            case 1800: // 1 mile 40 yards
                return "1\u00D9";
            case 1830: // 1 mile 70 yards
                return "1\u00DA";
        }

        if (distance >= 1760) {
            value = distance / 1760;
        } else {
            value = distance / 220;
        }

        let whole = Math.floor(value);
        let rem = value - whole;

        let fracStr = FRACTION_UNICODE_MAP[rem] || "";

        let result = String(whole) + fracStr;

        return result;
    }

    static getDistanceLength(distance: number): string {
        let result = this.getDistanceString(distance);

        if (distance === 1760) {
            result += " Mile";
        } else if (distance > 1760) {
            result += " Miles";
        } else result += " Furlongs";


        if (distance < 0) {
            result += "*";
        }

        return result;
    }   
    
    static getShortLength(distance: number | null): string {
        if (distance === null || distance === undefined) {
            return "";
        }

        let result = this.getDistanceString(distance);

        if (result.length === 1) {
            result += "f";
        }

        if (distance < 0) {
            result += "*";
        }

        return result;
    }



    static getAgeSexRestrictionString(restriction: string): string {
        const AGE_MAP: { [k: string]: string } = {
            A: "2",
            B: "3",
            C: "4",
            D: "5",
            E: "3&4",
            F: "4&5",
            G: "3,4&5",
            H: "All Ages",
        };

        const SEX_MAP: { [k: string]: string } = {
            M: "Mares & Fillies",
            C: "Colts & Geldings",
            F: "Fillies",
        };

        let result = AGE_MAP[restriction[0]];

        result += restriction[1] == "U" ? "&up" : "yo";

        if (restriction[2] == "N") return result;

        return result + " " + SEX_MAP[restriction[2]];
    }

    static capitalize(s: string): string {
        if (!s) return "";
        return s.toLowerCase().replace(/\w\S*/g, (txt, _offset, _str) => {
            // Capitalize parts separated by '/' as well (e.g. "smith/jones" -> "Smith/Jones")
            return txt
                .split("/")
                .map((part) => {
                    const m = part.match(/^([^a-zA-Z']*)([a-zA-Z']+)([^a-zA-Z']*)$/);
                    if (m) {
                        const prefix = m[1] || "";
                        const core = m[2] || "";
                        const suffix = m[3] || "";

                        if (this.TITLE_EXCEPTIONS.has(core)) {
                            return prefix + core + suffix;
                        }

                        return prefix + core.charAt(0).toUpperCase() + core.slice(1) + suffix;
                    }

                    return part.charAt(0).toUpperCase() + part.slice(1);
                })
                .join("/");
        });
    }

    static capitalizeWords(s: string): string {
        if (!s) return "";

        return s
            .trim()
            .split(/\s+/)
            .map((word) => {
                // support capitalization after '/' (e.g. Smith/Jones -> Smith/Jones)
                return word
                    .split("/")
                    .map((part) => {
                        const m = part.match(/^([^a-zA-Z']*)([a-zA-Z']+)([^a-zA-Z']*)$/);
                        if (m) {
                            const prefix = m[1] || "";
                            const core = m[2] || "";
                            const suffix = m[3] || "";

                            return prefix + core.charAt(0).toUpperCase() + core.slice(1).toLowerCase() + suffix;
                        }

                        return part.charAt(0).toUpperCase() + part.slice(1).toLowerCase();
                    })
                    .join("/");
            })
            .join(" ");
    }

    static commas(s: string): string {
        return s.replace(/;/g, ",");
    }

    static buildRaceConditions(race: Race): string {
        let result =
            race.race_conditions_line1 +
            race.race_conditions_line2 +
            race.race_conditions_line3 +
            race.race_conditions_line4 +
            race.race_conditions_line5 +
            race.race_conditions_line6;

        result = this.commas(result).trim();
        let pursePosition = result.toLowerCase().indexOf("purse");

        if (pursePosition >= 0) {
            result = result.slice(pursePosition);
        }

        return result;
    }

    static createOddsString(odds: number | null): string {
        if (odds === null || odds === undefined) {
            return "-";
        }

        for (let i = 1; i <= 10; i++) {
            let numerator = odds * i;

            if (Number.isInteger(numerator)) {
                return `${numerator}/${i}`;
            }
        }

        return "-";
    }

    static createPercentageString(num: number | null | undefined, denom: number | null | undefined): string {
        if (num === null || num === undefined) return "-";
        if (denom === null || denom === undefined) return "-";
        if (denom === 0) return "-";

        const pct = Math.round((num / denom) * 100);
        return `${pct}%`;
    }

    static createDollarString(num: number | null | undefined): string {
        if (num === null || num === undefined) return "-";

        const rounded = Math.round(num);
        return "$" + rounded.toLocaleString("en-US");
    }

    static createAgeString(birthMonth: number | null | undefined, birthYearTwoDigits: number | null | undefined): string {
        
        if (birthMonth === null || birthMonth === undefined) return "";
        if (birthYearTwoDigits === null || birthYearTwoDigits === undefined) return "";

        const month = Math.floor(birthMonth);
        const year = 2000 + Math.floor(birthYearTwoDigits);

        if (month < 1 || month > 12 || year < 0 || year > 9999) return "";

        const now = new Date();
        const years = now.getFullYear() - year;

        return `${years} (${this.MONTH_ABBREVIATIONS[month - 1]})`;
    }

    static formatOneDecimal(value: number | null | undefined): string {
        if (value === null || value === undefined) return "";
        if (Number.isNaN(value)) return "";

        return value.toFixed(1);
    }

    static capitalizeFullName(input: string): string {
        if (!input) return "";

        const lowercaseParticles = new Set([
            "van",
            "von",
            "de",
            "del",
            "da",
            "di",
            "la",
            "le",
            "du",
            "st",
            "der",
        ]);
       const handleMcMac = (word: string): string => {
            const lower = word.toLowerCase();

            if (lower.startsWith("mc") && lower.length > 2) {
                return "Mc" + lower.charAt(2).toUpperCase() + lower.slice(3);
            }

            if (lower.startsWith("mac") && lower.length > 3) {
                return "Mac" + lower.charAt(3).toUpperCase() + lower.slice(4);
            }

            return this.capitalize(word);
        };

        const formatSurnamePart = (part: string): string =>
            part
                .split("-")
                .map((seg) =>
                    seg
                        .split("'")
                        .map((sub) => handleMcMac(sub))
                        .join("'")
                )
                .join("-");

        const tokens = input.trim().split(/\s+/);
        if (tokens.length === 1) {
            return formatSurnamePart(tokens[0]);
        }

        const lastIndex = tokens.length - 1;

        return tokens
            .map((token, index) => {
                if (/^[a-z]\.?$/i.test(token)) {
                    return token.toUpperCase().endsWith(".")
                        ? token.toUpperCase()
                        : token.toUpperCase() + ".";
                }

                if (index >= lastIndex) {
                    return formatSurnamePart(token);
                }

                const lower = token.toLowerCase();

                if (index === lastIndex - 1 && lowercaseParticles.has(lower)) {
                    return lower;
                }

                return this.capitalize(token);
            })
            .join(" ");
    }

     static formatDateShort(dateStr: string): string {
        if (!dateStr) return "";

        const m = dateStr.match(/^(\d{1,2})\/(\d{1,2})\/(\d{2,4})$/);
        if (!m) return "";

        const month = parseInt(m[1], 10);
        const day = parseInt(m[2], 10);
        let yearStr = m[3];

        if (Number.isNaN(month) || Number.isNaN(day)) return "";

       if (yearStr.length === 2) {
            yearStr = "20" + yearStr;
        }

        const year = parseInt(yearStr, 10);
        if (Number.isNaN(year)) return "";
        if (month < 1 || month > 12) return "";
        if (day < 1 || day > 31) return "";

        const monthAbbr = this.MONTH_ABBREVIATIONS[month - 1] || "";
        const yy = String(year).slice(-2);

        return `${String(day).padStart(2, "0")}${monthAbbr}${yy}`;
    }  
    
    static parseNumberOrNull(value: any): number | null {
        if (value === null || value === undefined) return null;
        const s = String(value).trim();
        if (s === "") return null;
        const cleaned = s.replace(/[^0-9.-]/g, "");
        if (cleaned === "") return null;
        const n = Number(cleaned);
        return Number.isFinite(n) ? n : null;
    }

    static getSurfaceString(pp: PastPerformance) : string {
        let result = "";

        if (pp.code_for_prior_races.toLowerCase() === "x") {
            return "\u00f2";
        }

        if (pp.previous_all_weather_surface_indicator === "A") {
            return "\u00f1";
        }

        switch (pp.surface) {
            case "T":
                return "\u00db";
            case "t":
                return "\u00dc"
            case "d":
                return "\u00dd";
        }

        return result;       
    }

    static getFractionalTimeString(time: number | null): [string, string] {
        if (time === null || time === undefined) {
            return ["", ""];
        }

        if (!Number.isFinite(time)) return ["", ""];

        let totalSeconds = Math.abs(time);

        let minutes = Math.floor(totalSeconds / 60);
        let remaining = totalSeconds - minutes * 60;

        let seconds = Math.floor(remaining);
        let fractional = remaining - seconds;

        let fifths = Math.floor(fractional * 5);

        let main: string;
        if (minutes === 0) {
            main = ":" +String(seconds).padStart(2, "0");
        } else {
            main = `${minutes}:${String(seconds).padStart(2, "0")}`;
        }

        return [main, fifths === 0 ? " " : String(fifths)];
    }
}
