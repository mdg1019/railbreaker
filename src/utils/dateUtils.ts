/**
 * Parse a date string in `MM/DD/YYYY` format and return the weekday name.
 * Uses `Date.UTC` so the calculation is done in UTC.
 *
 * @param dateStr - date string like "12/31/2025"
 * @param locale - optional locale for weekday name (defaults to 'en-US')
 * @returns weekday name (e.g., 'Saturday') or null if input invalid
 */
export function mmddyyyyToWeekday(dateStr: string, locale = 'en-US'): string | null {
  const m = dateStr.match(/^(\d{1,2})\/(\d{1,2})\/(\d{4})$/);
  if (!m) return null;
  const month = Number(m[1]);
  const day = Number(m[2]);
  const year = Number(m[3]);
  if (Number.isNaN(month) || Number.isNaN(day) || Number.isNaN(year)) return null;
  if (month < 1 || month > 12) return null;
  if (day < 1 || day > 31) return null;

  const utcMillis = Date.UTC(year, month - 1, day);
  const d = new Date(utcMillis);

  try {
    return new Intl.DateTimeFormat(locale, { weekday: 'long', timeZone: 'UTC' }).format(d);
  } catch (e) {
    const names = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
    return names[d.getUTCDay()] || null;
  }
}

export function mmddyyyyToLongDate(dateStr: string, locale = 'en-US'): string | null {
  const m = dateStr.match(/^(\d{1,2})\/(\d{1,2})\/(\d{4})$/);
  if (!m) return null;
  const month = Number(m[1]);
  const day = Number(m[2]);
  const year = Number(m[3]);
  if (Number.isNaN(month) || Number.isNaN(day) || Number.isNaN(year)) return null;
  if (month < 1 || month > 12) return null;
  if (day < 1 || day > 31) return null;

  const utcMillis = Date.UTC(year, month - 1, day);
  const d = new Date(utcMillis);

  try {
    return new Intl.DateTimeFormat(locale, { month: 'long', day: 'numeric', year: 'numeric', timeZone: 'UTC' }).format(d);
  } catch (e) {
    const months = [
      'January', 'February', 'March', 'April', 'May', 'June',
      'July', 'August', 'September', 'October', 'November', 'December'
    ];
    const mon = months[d.getUTCMonth()] || '';
    const dy = d.getUTCDate();
    const yr = d.getUTCFullYear();
    return mon ? `${mon} ${dy}, ${yr}` : null;
  }
}
