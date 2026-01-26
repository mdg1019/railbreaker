export class Racecard {
  id: number;
  zip_file_name: string;
  track: string;
  date: string;
  long_date: string;
  races: Race[];

  constructor(data: any = {}) {
    this.id = data.id ?? 0;
    this.zip_file_name = data.zip_file_name ?? '';
    this.track = data.track ?? '';
    this.date = data.date ?? '';
    this.long_date = data.long_date ?? '';
    this.races = (data.races ?? []).map((r: any) =>
      r instanceof Race ? r : Race.fromObject(r)
    );
  }

  static fromObject(obj: any): Racecard {
    return new Racecard({
      id: obj.id ?? 0,
      zip_file_name: obj.zip_file_name,
      track: obj.track,
      date: obj.date,
      long_date: obj.long_date,
      races: obj.races?.map((r: any) => Race.fromObject(r)) ?? []
    });
  }

  toObject(): any {
    return {
      id: this.id,
      zip_file_name: this.zip_file_name,
      track: this.track,
      date: this.date,
      long_date: this.long_date,
      races: this.races.map(r => r.toObject())
    };
  }
}

export class Race {
  id: number;
  racecard_id: number;
  race_number: number | null;
  distance: number | null;
  surface: string;
  race_type: string;
  age_sex_restrictions: string;
  todays_race_classification: string;
  purse: number | null;
  claiming_price: number | null;
  track_record: number | null;
  race_conditions: string;
  todays_lasix_list: string;
  todays_bute_list: string;
  todays_coupled_list: string;
  todays_mutuel_list: string;
  simulcast_host_track_code: string;
  simulcast_host_track_race_number: number | null;
  all_weather_surface_flag: string;
  race_conditions_line1: string;
  race_conditions_line2: string;
  race_conditions_line3: string;
  race_conditions_line4: string;
  race_conditions_line5: string;
  race_conditions_line6: string;
  low_claiming_price: number | null;
  statebred_flag: string;
  wager_type_line1: string;
  wager_type_line2: string;
  wager_type_line3: string;
  wager_type_line4: string;
  wager_type_line5: string;
  wager_type_line6: string;
  wager_type_line7: string;
  wager_type_line8: string;
  wager_type_line9: string;
  two_f_bris_pace_par: number | null;
  four_f_bris_pace_par: number | null;
  six_f_bris_pace_par: number | null;
  bris_speed_for_class: number | null;
  post_times: string;
  post_time_pacific_military: string;
  bris_late_pace_par: number | null;
  todays_equibase_abbreviated_race_conditions: string;
  horses: Horse[];

  constructor(data: any = {}) {
    this.id = data.id ?? 0;
    this.racecard_id = data.racecard_id ?? 0;
    this.race_number = data.race_number ?? null;
    this.distance = data.distance ?? null;
    this.surface = data.surface ?? '';
    this.race_type = data.race_type ?? '';
    this.age_sex_restrictions = data.age_sex_restrictions ?? '';
    this.todays_race_classification = data.todays_race_classification ?? '';
    this.purse = data.purse ?? null;
    this.claiming_price = data.claiming_price ?? null;
    this.track_record = data.track_record ?? null;
    this.race_conditions = data.race_conditions ?? '';
    this.todays_lasix_list = data.todays_lasix_list ?? '';
    this.todays_bute_list = data.todays_bute_list ?? '';
    this.todays_coupled_list = data.todays_coupled_list ?? '';
    this.todays_mutuel_list = data.todays_mutuel_list ?? '';
    this.simulcast_host_track_code = data.simulcast_host_track_code ?? '';
    this.simulcast_host_track_race_number = data.simulcast_host_track_race_number ?? null;
    this.all_weather_surface_flag = data.all_weather_surface_flag ?? '';
    this.race_conditions_line1 = data.race_conditions_line1 ?? '';
    this.race_conditions_line2 = data.race_conditions_line2 ?? '';
    this.race_conditions_line3 = data.race_conditions_line3 ?? '';
    this.race_conditions_line4 = data.race_conditions_line4 ?? '';
    this.race_conditions_line5 = data.race_conditions_line5 ?? '';
    this.race_conditions_line6 = data.race_conditions_line6 ?? '';
    this.low_claiming_price = data.low_claiming_price ?? null;
    this.statebred_flag = data.statebred_flag ?? '';
    this.wager_type_line1 = data.wager_type_line1 ?? '';
    this.wager_type_line2 = data.wager_type_line2 ?? '';
    this.wager_type_line3 = data.wager_type_line3 ?? '';
    this.wager_type_line4 = data.wager_type_line4 ?? '';
    this.wager_type_line5 = data.wager_type_line5 ?? '';
    this.wager_type_line6 = data.wager_type_line6 ?? '';
    this.wager_type_line7 = data.wager_type_line7 ?? '';
    this.wager_type_line8 = data.wager_type_line8 ?? '';
    this.wager_type_line9 = data.wager_type_line9 ?? '';
    this.two_f_bris_pace_par = data.two_f_bris_pace_par ?? null;
    this.four_f_bris_pace_par = data.four_f_bris_pace_par ?? null;
    this.six_f_bris_pace_par = data.six_f_bris_pace_par ?? null;
    this.bris_speed_for_class = data.bris_speed_for_class ?? null;
    this.bris_late_pace_par = data.bris_late_pace_par ?? null;
    this.post_times = data.post_times ?? '';
    this.post_time_pacific_military = data.post_time_pacific_military ?? '';
    this.todays_equibase_abbreviated_race_conditions = data.todays_equibase_abbreviated_race_conditions ?? '';
    this.horses = (data.horses ?? []).map((h: any) =>
      h instanceof Horse ? h : Horse.fromObject(h)
    );
  }

  static fromObject(obj: any): Race {
    return new Race({
      ...obj,
      horses: obj.horses?.map((h: any) => Horse.fromObject(h)) ?? []
    });
  }

  toObject(): any {
    return {
      id: this.id,
      racecard_id: this.racecard_id,
      race_number: this.race_number,
      distance: this.distance,
      surface: this.surface,
      race_type: this.race_type,
      age_sex_restrictions: this.age_sex_restrictions,
      todays_race_classification: this.todays_race_classification,
      purse: this.purse,
      claiming_price: this.claiming_price,
      track_record: this.track_record,
      race_conditions: this.race_conditions,
      todays_lasix_list: this.todays_lasix_list,
      todays_bute_list: this.todays_bute_list,
      todays_coupled_list: this.todays_coupled_list,
      todays_mutuel_list: this.todays_mutuel_list,
      simulcast_host_track_code: this.simulcast_host_track_code,
      simulcast_host_track_race_number: this.simulcast_host_track_race_number,
      all_weather_surface_flag: this.all_weather_surface_flag,
      race_conditions_line1: this.race_conditions_line1,
      race_conditions_line2: this.race_conditions_line2,
      race_conditions_line3: this.race_conditions_line3,
      race_conditions_line4: this.race_conditions_line4,
      race_conditions_line5: this.race_conditions_line5,
      race_conditions_line6: this.race_conditions_line6,
      low_claiming_price: this.low_claiming_price,
      statebred_flag: this.statebred_flag,
      wager_type_line1: this.wager_type_line1,
      wager_type_line2: this.wager_type_line2,
      wager_type_line3: this.wager_type_line3,
      wager_type_line4: this.wager_type_line4,
      wager_type_line5: this.wager_type_line5,
      wager_type_line6: this.wager_type_line6,
      wager_type_line7: this.wager_type_line7,
      wager_type_line8: this.wager_type_line8,
      wager_type_line9: this.wager_type_line9,
      two_f_bris_pace_par: this.two_f_bris_pace_par,
      four_f_bris_pace_par: this.four_f_bris_pace_par,
      six_f_bris_pace_par: this.six_f_bris_pace_par,
      bris_speed_for_class: this.bris_speed_for_class,
      bris_late_pace_par: this.bris_late_pace_par,
      post_times: this.post_times,
      post_time_pacific_military: this.post_time_pacific_military,
      todays_equibase_abbreviated_race_conditions: this.todays_equibase_abbreviated_race_conditions,
      horses: this.horses.map(h => h.toObject())
    };
  }
}

export class Horse {
  id: number;
  race_id: number;
  post_position: number | null;
  entry: string;
  claiming_price_of_horse: number | null;
  breed_type: string;
  todays_nasal_strip_change: number | null;
  todays_trainer: string;
  trainer_starts: number | null;
  trainer_wins: number | null;
  trainer_places: number | null;
  trainer_shows: number | null;
  todays_jockey: string;
  apprentice_weight_allowance: number | null;
  jockey_starts: number | null;
  jockey_wins: number | null;
  jockey_places: number | null;
  jockey_shows: number | null;
  todays_owner: string;
  owners_silks: string;
  main_track_only_ae_indicator: string;
  program_number: string;
  morning_line_odds: number | null;
  horse_name: string;
  year_of_birth: number | null;
  horses_foaling_month: number | null;
  sex: string;
  horses_color: string;
  weight: number | null;
  sire: string;
  sires_sire: string;
  dam: string;
  dams_sire: string;
  breeder: string;
  state_country_where_bred: string;
  program_post_position: string;
  todays_medication_new: number | null;
  todays_medication_old: number | null;
  equipment_change: number | null;
  lifetime_record_todays_distance_starts: number | null;
  lifetime_record_todays_distance_wins: number | null;
  lifetime_record_todays_distance_places: number | null;
  lifetime_record_todays_distance_shows: number | null;
  lifetime_record_todays_distance_earnings: number | null;
  lifetime_record_todays_track_starts: number | null;
  lifetime_record_todays_track_wins: number | null;
  lifetime_record_todays_track_places: number | null;
  lifetime_record_todays_track_shows: number | null;
  lifetime_record_todays_track_earnings: number | null;
  lifetime_record_turf_starts: number | null;
  lifetime_record_turf_wins: number | null;
  lifetime_record_turf_places: number | null;
  lifetime_record_turf_shows: number | null;
  lifetime_record_turf_earnings: number | null;
  lifetime_record_wet_starts: number | null;
  lifetime_record_wet_wins: number | null;
  lifetime_record_wet_places: number | null;
  lifetime_record_wet_shows: number | null;
  lifetime_record_wet_earnings: number | null;
  current_year_record_year: number | null;
  current_year_record_starts: number | null;
  current_year_record_wins: number | null;
  current_year_record_places: number | null;
  current_year_record_shows: number | null;
  current_year_record_earnings: number | null;
  previous_year_record_year: number | null;
  previous_year_record_starts: number | null;
  previous_year_record_wins: number | null;
  previous_year_record_places: number | null;
  previous_year_record_shows: number | null;
  previous_year_record_earnings: number | null;
  lifetime_record_starts: number | null;
  lifetime_record_wins: number | null;
  lifetime_record_places: number | null;
  lifetime_record_shows: number | null;
  lifetime_record_earnings: number | null;
  bris_run_style: string;
  quirin_speed_points: number | null;
  trainer_jockey_combo_starts: number | null;
  trainer_jockey_combo_wins: number | null;
  trainer_jockey_combo_places: number | null;
  trainer_jockey_combo_shows: number | null;
  trainer_jockey_combo_roi: number | null;
  days_since_last_race: number | null;
  lifetime_all_weather_starts: number | null;
  lifetime_all_weather_wins: number | null;
  lifetime_all_weather_places: number | null;
  lifetime_all_weather_shows: number | null;
  lifetime_all_weather_earnings: number | null;
  best_bris_speed_all_weather_surface: number | null;
  bris_prime_power_rating: number | null;
  trainer_starts_current_year: number | null;
  trainer_wins_current_year: number | null;
  trainer_places_current_year: number | null;
  trainer_shows_current_year: number | null;
  trainer_roi_current_year: number | null;
  trainer_starts_previous_year: number | null;
  trainer_wins_previous_year: number | null;
  trainer_places_previous_year: number | null;
  trainer_shows_previous_year: number | null;
  trainer_roi_previous_year: number | null;
  jockey_starts_current_year: number | null;
  jockey_wins_current_year: number | null;
  jockey_places_current_year: number | null;
  jockey_shows_current_year: number | null;
  jockey_roi_current_year: number | null;
  jockey_starts_previous_year: number | null;
  jockey_wins_previous_year: number | null;
  jockey_places_previous_year: number | null;
  jockey_shows_previous_year: number | null;
  jockey_roi_previous_year: number | null;
  sire_stud_fee: number | null;
  best_bris_speed_fast_track: number | null;
  best_bris_speed_turf: number | null;
  best_bris_speed_off_track: number | null;
  best_bris_speed_distance: number | null;
  auction_price: number | null;
  where_when_sold_at_auction: string;
  bris_dirt_pedigree_rating: string;
  bris_mud_pedigree_rating: string;
  bris_turf_pedigree_rating: string;
  bris_distance_pedigree_rating: string;
  best_bris_speed_life: number | null;
  best_bris_speed_most_recent_year: number | null;
  best_bris_speed_2nd_most_recent_year: number | null;
  best_bris_speed_todays_track: number | null;
  starts_fast_dirt: number | null;
  wins_fast_dirt: number | null;
  places_fast_dirt: number | null;
  shows_fast_dirt: number | null;
  earnings_fast_dirt: number | null;
  jockey_distance_turf_label: string;
  jockey_distance_turf_starts: number | null;
  jockey_distance_turf_wins: number | null;
  jockey_distance_turf_places: number | null;
  jockey_distance_turf_shows: number | null;
  jockey_distance_turf_roi: number | null;
  jockey_distance_turf_earnings: number | null;
  trainer_jockey_combo_starts_meet: number | null;
  trainer_jockey_combo_wins_meet: number | null;
  trainer_jockey_combo_places_meet: number | null;
  trainer_jockey_combo_shows_meet: number | null;
  trainer_jockey_combo_roi_meet: number | null;
  note: string = '';
  workouts: Workout[];
  past_performances: PastPerformance[];
  key_trainer_stats: KeyTrainerStat[];

  constructor(data: any = {}) {
    this.id = data.id ?? 0;
    this.race_id = data.race_id ?? 0;
    this.post_position = data.post_position ?? null;
    this.entry = data.entry ?? '';
    this.claiming_price_of_horse = data.claiming_price_of_horse ?? null;
    this.breed_type = data.breed_type ?? '';
    this.todays_nasal_strip_change = data.todays_nasal_strip_change ?? null;
    this.todays_trainer = data.todays_trainer ?? '';
    this.trainer_starts = data.trainer_starts ?? null;
    this.trainer_wins = data.trainer_wins ?? null;
    this.trainer_places = data.trainer_places ?? null;
    this.trainer_shows = data.trainer_shows ?? null;
    this.todays_jockey = data.todays_jockey ?? '';
    this.apprentice_weight_allowance = data.apprentice_weight_allowance ?? null;
    this.jockey_starts = data.jockey_starts ?? null;
    this.jockey_wins = data.jockey_wins ?? null;
    this.jockey_places = data.jockey_places ?? null;
    this.jockey_shows = data.jockey_shows ?? null;
    this.todays_owner = data.todays_owner ?? '';
    this.owners_silks = data.owners_silks ?? '';
    this.main_track_only_ae_indicator = data.main_track_only_ae_indicator ?? '';
    this.program_number = data.program_number ?? '';
    this.morning_line_odds = data.morning_line_odds ?? null;
    this.horse_name = data.horse_name ?? '';
    this.year_of_birth = data.year_of_birth ?? null;
    this.horses_foaling_month = data.horses_foaling_month ?? null;
    this.sex = data.sex ?? '';
    this.horses_color = data.horses_color ?? '';
    this.weight = data.weight ?? null;
    this.sire = data.sire ?? '';
    this.sires_sire = data.sires_sire ?? '';
    this.dam = data.dam ?? '';
    this.dams_sire = data.dams_sire ?? '';
    this.breeder = data.breeder ?? '';
    this.state_country_where_bred = data.state_country_where_bred ?? '';
    this.program_post_position = data.program_post_position ?? '';
    this.todays_medication_new = data.todays_medication_new ?? null;
    this.todays_medication_old = data.todays_medication_old ?? null;
    this.equipment_change = data.equipment_change ?? null;
    this.lifetime_record_todays_distance_starts = data.lifetime_record_todays_distance_starts ?? null;
    this.lifetime_record_todays_distance_wins = data.lifetime_record_todays_distance_wins ?? null;
    this.lifetime_record_todays_distance_places = data.lifetime_record_todays_distance_places ?? null;
    this.lifetime_record_todays_distance_shows = data.lifetime_record_todays_distance_shows ?? null;
    this.lifetime_record_todays_distance_earnings = data.lifetime_record_todays_distance_earnings ?? null;
    this.lifetime_record_todays_track_starts = data.lifetime_record_todays_track_starts ?? null;
    this.lifetime_record_todays_track_wins = data.lifetime_record_todays_track_wins ?? null;
    this.lifetime_record_todays_track_places = data.lifetime_record_todays_track_places ?? null;
    this.lifetime_record_todays_track_shows = data.lifetime_record_todays_track_shows ?? null;
    this.lifetime_record_todays_track_earnings = data.lifetime_record_todays_track_earnings ?? null;
    this.lifetime_record_turf_starts = data.lifetime_record_turf_starts ?? null;
    this.lifetime_record_turf_wins = data.lifetime_record_turf_wins ?? null;
    this.lifetime_record_turf_places = data.lifetime_record_turf_places ?? null;
    this.lifetime_record_turf_shows = data.lifetime_record_turf_shows ?? null;
    this.lifetime_record_turf_earnings = data.lifetime_record_turf_earnings ?? null;
    this.lifetime_record_wet_starts = data.lifetime_record_wet_starts ?? null;
    this.lifetime_record_wet_wins = data.lifetime_record_wet_wins ?? null;
    this.lifetime_record_wet_places = data.lifetime_record_wet_places ?? null;
    this.lifetime_record_wet_shows = data.lifetime_record_wet_shows ?? null;
    this.lifetime_record_wet_earnings = data.lifetime_record_wet_earnings ?? null;
    this.current_year_record_year = data.current_year_record_year ?? null;
    this.current_year_record_starts = data.current_year_record_starts ?? null;
    this.current_year_record_wins = data.current_year_record_wins ?? null;
    this.current_year_record_places = data.current_year_record_places ?? null;
    this.current_year_record_shows = data.current_year_record_shows ?? null;
    this.current_year_record_earnings = data.current_year_record_earnings ?? null;
    this.previous_year_record_year = data.previous_year_record_year ?? null;
    this.previous_year_record_starts = data.previous_year_record_starts ?? null;
    this.previous_year_record_wins = data.previous_year_record_wins ?? null;
    this.previous_year_record_places = data.previous_year_record_places ?? null;
    this.previous_year_record_shows = data.previous_year_record_shows ?? null;
    this.previous_year_record_earnings = data.previous_year_record_earnings ?? null;
    this.lifetime_record_starts = data.lifetime_record_starts ?? null;
    this.lifetime_record_wins = data.lifetime_record_wins ?? null;
    this.lifetime_record_places = data.lifetime_record_places ?? null;
    this.lifetime_record_shows = data.lifetime_record_shows ?? null;
    this.lifetime_record_earnings = data.lifetime_record_earnings ?? null;
    this.bris_run_style = data.bris_run_style ?? '';
    this.quirin_speed_points = data.quirin_speed_points ?? null;
    this.trainer_jockey_combo_starts = data.trainer_jockey_combo_starts ?? null;
    this.trainer_jockey_combo_wins = data.trainer_jockey_combo_wins ?? null;
    this.trainer_jockey_combo_places = data.trainer_jockey_combo_places ?? null;
    this.trainer_jockey_combo_shows = data.trainer_jockey_combo_shows ?? null;
    this.trainer_jockey_combo_roi = data.trainer_jockey_combo_roi ?? null;
    this.days_since_last_race = data.days_since_last_race ?? null;
    this.lifetime_all_weather_starts = data.lifetime_all_weather_starts ?? null;
    this.lifetime_all_weather_wins = data.lifetime_all_weather_wins ?? null;
    this.lifetime_all_weather_places = data.lifetime_all_weather_places ?? null;
    this.lifetime_all_weather_shows = data.lifetime_all_weather_shows ?? null;
    this.lifetime_all_weather_earnings = data.lifetime_all_weather_earnings ?? null;
    this.best_bris_speed_all_weather_surface = data.best_bris_speed_all_weather_surface ?? null;
    this.bris_prime_power_rating = data.bris_prime_power_rating ?? null;
    this.trainer_starts_current_year = data.trainer_starts_current_year ?? null;
    this.trainer_wins_current_year = data.trainer_wins_current_year ?? null;
    this.trainer_places_current_year = data.trainer_places_current_year ?? null;
    this.trainer_shows_current_year = data.trainer_shows_current_year ?? null;
    this.trainer_roi_current_year = data.trainer_roi_current_year ?? null;
    this.trainer_starts_previous_year = data.trainer_starts_previous_year ?? null;
    this.trainer_wins_previous_year = data.trainer_wins_previous_year ?? null;
    this.trainer_places_previous_year = data.trainer_places_previous_year ?? null;
    this.trainer_shows_previous_year = data.trainer_shows_previous_year ?? null;
    this.trainer_roi_previous_year = data.trainer_roi_previous_year ?? null;
    this.jockey_starts_current_year = data.jockey_starts_current_year ?? null;
    this.jockey_wins_current_year = data.jockey_wins_current_year ?? null;
    this.jockey_places_current_year = data.jockey_places_current_year ?? null;
    this.jockey_shows_current_year = data.jockey_shows_current_year ?? null;
    this.jockey_roi_current_year = data.jockey_roi_current_year ?? null;
    this.jockey_starts_previous_year = data.jockey_starts_previous_year ?? null;
    this.jockey_wins_previous_year = data.jockey_wins_previous_year ?? null;
    this.jockey_places_previous_year = data.jockey_places_previous_year ?? null;
    this.jockey_shows_previous_year = data.jockey_shows_previous_year ?? null;
    this.jockey_roi_previous_year = data.jockey_roi_previous_year ?? null;
    this.sire_stud_fee = data.sire_stud_fee ?? null;
    this.best_bris_speed_fast_track = data.best_bris_speed_fast_track ?? null;
    this.best_bris_speed_turf = data.best_bris_speed_turf ?? null;
    this.best_bris_speed_off_track = data.best_bris_speed_off_track ?? null;
    this.best_bris_speed_distance = data.best_bris_speed_distance ?? null;
    this.auction_price = data.auction_price ?? null;
    this.where_when_sold_at_auction = data.where_when_sold_at_auction ?? '';
    this.bris_dirt_pedigree_rating = data.bris_dirt_pedigree_rating ?? '';
    this.bris_mud_pedigree_rating = data.bris_mud_pedigree_rating ?? '';
    this.bris_turf_pedigree_rating = data.bris_turf_pedigree_rating ?? '';
    this.bris_distance_pedigree_rating = data.bris_distance_pedigree_rating ?? '';
    this.best_bris_speed_life = data.best_bris_speed_life ?? null;
    this.best_bris_speed_most_recent_year = data.best_bris_speed_most_recent_year ?? null;
    this.best_bris_speed_2nd_most_recent_year = data.best_bris_speed_2nd_most_recent_year ?? null;
    this.best_bris_speed_todays_track = data.best_bris_speed_todays_track ?? null;
    this.starts_fast_dirt = data.starts_fast_dirt ?? null;
    this.wins_fast_dirt = data.wins_fast_dirt ?? null;
    this.places_fast_dirt = data.places_fast_dirt ?? null;
    this.shows_fast_dirt = data.shows_fast_dirt ?? null;
    this.earnings_fast_dirt = data.earnings_fast_dirt ?? null;
    this.jockey_distance_turf_label = data.jockey_distance_turf_label ?? '';
    this.jockey_distance_turf_starts = data.jockey_distance_turf_starts ?? null;
    this.jockey_distance_turf_wins = data.jockey_distance_turf_wins ?? null;
    this.jockey_distance_turf_places = data.jockey_distance_turf_places ?? null;
    this.jockey_distance_turf_shows = data.jockey_distance_turf_shows ?? null;
    this.jockey_distance_turf_roi = data.jockey_distance_turf_roi ?? null;
    this.jockey_distance_turf_earnings = data.jockey_distance_turf_earnings ?? null;
    this.trainer_jockey_combo_starts_meet = data.trainer_jockey_combo_starts_meet ?? null;
    this.trainer_jockey_combo_wins_meet = data.trainer_jockey_combo_wins_meet ?? null;
    this.trainer_jockey_combo_places_meet = data.trainer_jockey_combo_places_meet ?? null;
    this.trainer_jockey_combo_shows_meet = data.trainer_jockey_combo_shows_meet ?? null;
    this.trainer_jockey_combo_roi_meet = data.trainer_jockey_combo_roi_meet ?? null;
    this.note = data.note ?? '';
    this.workouts = (data.workouts ?? []).map((w: any) =>
      w instanceof Workout ? w : Workout.fromObject(w)
    );
    this.past_performances = (data.past_performances ?? []).map((p: any) =>
      p instanceof PastPerformance ? p : PastPerformance.fromObject(p)
    );
    this.key_trainer_stats = (data.key_trainer_stats ?? []).map((k: any) =>
      k instanceof KeyTrainerStat ? k : KeyTrainerStat.fromObject(k)
    );
  }

  static fromObject(obj: any): Horse {
    return new Horse({
      ...obj,
      workouts: obj.workouts?.map((w: any) => Workout.fromObject(w)) ?? [],
      past_performances: obj.past_performances?.map((p: any) => PastPerformance.fromObject(p)) ?? [],
      key_trainer_stats: obj.key_trainer_stats?.map((k: any) => KeyTrainerStat.fromObject(k)) ?? []
    });
  }

  toObject(): any {
    return {
      id: this.id,
      race_id: this.race_id,
      post_position: this.post_position,
      entry: this.entry,
      claiming_price_of_horse: this.claiming_price_of_horse,
      breed_type: this.breed_type,
      todays_nasal_strip_change: this.todays_nasal_strip_change,
      todays_trainer: this.todays_trainer,
      trainer_starts: this.trainer_starts,
      trainer_wins: this.trainer_wins,
      trainer_places: this.trainer_places,
      trainer_shows: this.trainer_shows,
      todays_jockey: this.todays_jockey,
      apprentice_weight_allowance: this.apprentice_weight_allowance,
      jockey_starts: this.jockey_starts,
      jockey_wins: this.jockey_wins,
      jockey_places: this.jockey_places,
      jockey_shows: this.jockey_shows,
      todays_owner: this.todays_owner,
      owners_silks: this.owners_silks,
      main_track_only_ae_indicator: this.main_track_only_ae_indicator,
      program_number: this.program_number,
      morning_line_odds: this.morning_line_odds,
      horse_name: this.horse_name,
      year_of_birth: this.year_of_birth,
      horses_foaling_month: this.horses_foaling_month,
      sex: this.sex,
      horses_color: this.horses_color,
      weight: this.weight,
      sire: this.sire,
      sires_sire: this.sires_sire,
      dam: this.dam,
      dams_sire: this.dams_sire,
      breeder: this.breeder,
      state_country_where_bred: this.state_country_where_bred,
      program_post_position: this.program_post_position,
      todays_medication_new: this.todays_medication_new,
      todays_medication_old: this.todays_medication_old,
      equipment_change: this.equipment_change,
      lifetime_record_todays_distance_starts: this.lifetime_record_todays_distance_starts,
      lifetime_record_todays_distance_wins: this.lifetime_record_todays_distance_wins,
      lifetime_record_todays_distance_places: this.lifetime_record_todays_distance_places,
      lifetime_record_todays_distance_shows: this.lifetime_record_todays_distance_shows,
      lifetime_record_todays_distance_earnings: this.lifetime_record_todays_distance_earnings,
      lifetime_record_todays_track_starts: this.lifetime_record_todays_track_starts,
      lifetime_record_todays_track_wins: this.lifetime_record_todays_track_wins,
      lifetime_record_todays_track_places: this.lifetime_record_todays_track_places,
      lifetime_record_todays_track_shows: this.lifetime_record_todays_track_shows,
      lifetime_record_todays_track_earnings: this.lifetime_record_todays_track_earnings,
      lifetime_record_turf_starts: this.lifetime_record_turf_starts,
      lifetime_record_turf_wins: this.lifetime_record_turf_wins,
      lifetime_record_turf_places: this.lifetime_record_turf_places,
      lifetime_record_turf_shows: this.lifetime_record_turf_shows,
      lifetime_record_turf_earnings: this.lifetime_record_turf_earnings,
      lifetime_record_wet_starts: this.lifetime_record_wet_starts,
      lifetime_record_wet_wins: this.lifetime_record_wet_wins,
      lifetime_record_wet_places: this.lifetime_record_wet_places,
      lifetime_record_wet_shows: this.lifetime_record_wet_shows,
      lifetime_record_wet_earnings: this.lifetime_record_wet_earnings,
      current_year_record_year: this.current_year_record_year,
      current_year_record_starts: this.current_year_record_starts,
      current_year_record_wins: this.current_year_record_wins,
      current_year_record_places: this.current_year_record_places,
      current_year_record_shows: this.current_year_record_shows,
      current_year_record_earnings: this.current_year_record_earnings,
      previous_year_record_year: this.previous_year_record_year,
      previous_year_record_starts: this.previous_year_record_starts,
      previous_year_record_wins: this.previous_year_record_wins,
      previous_year_record_places: this.previous_year_record_places,
      previous_year_record_shows: this.previous_year_record_shows,
      previous_year_record_earnings: this.previous_year_record_earnings,
      lifetime_record_starts: this.lifetime_record_starts,
      lifetime_record_wins: this.lifetime_record_wins,
      lifetime_record_places: this.lifetime_record_places,
      lifetime_record_shows: this.lifetime_record_shows,
      lifetime_record_earnings: this.lifetime_record_earnings,
      bris_run_style: this.bris_run_style,
      quirin_speed_points: this.quirin_speed_points,
      trainer_jockey_combo_starts: this.trainer_jockey_combo_starts,
      trainer_jockey_combo_wins: this.trainer_jockey_combo_wins,
      trainer_jockey_combo_places: this.trainer_jockey_combo_places,
      trainer_jockey_combo_shows: this.trainer_jockey_combo_shows,
      trainer_jockey_combo_roi: this.trainer_jockey_combo_roi,
      days_since_last_race: this.days_since_last_race,
      lifetime_all_weather_starts: this.lifetime_all_weather_starts,
      lifetime_all_weather_wins: this.lifetime_all_weather_wins,
      lifetime_all_weather_places: this.lifetime_all_weather_places,
      lifetime_all_weather_shows: this.lifetime_all_weather_shows,
      lifetime_all_weather_earnings: this.lifetime_all_weather_earnings,
      best_bris_speed_all_weather_surface: this.best_bris_speed_all_weather_surface,
      bris_prime_power_rating: this.bris_prime_power_rating,
      trainer_starts_current_year: this.trainer_starts_current_year,
      trainer_wins_current_year: this.trainer_wins_current_year,
      trainer_places_current_year: this.trainer_places_current_year,
      trainer_shows_current_year: this.trainer_shows_current_year,
      trainer_roi_current_year: this.trainer_roi_current_year,
      trainer_starts_previous_year: this.trainer_starts_previous_year,
      trainer_wins_previous_year: this.trainer_wins_previous_year,
      trainer_places_previous_year: this.trainer_places_previous_year,
      trainer_shows_previous_year: this.trainer_shows_previous_year,
      trainer_roi_previous_year: this.trainer_roi_previous_year,
      jockey_starts_current_year: this.jockey_starts_current_year,
      jockey_wins_current_year: this.jockey_wins_current_year,
      jockey_places_current_year: this.jockey_places_current_year,
      jockey_shows_current_year: this.jockey_shows_current_year,
      jockey_roi_current_year: this.jockey_roi_current_year,
      jockey_starts_previous_year: this.jockey_starts_previous_year,
      jockey_wins_previous_year: this.jockey_wins_previous_year,
      jockey_places_previous_year: this.jockey_places_previous_year,
      jockey_shows_previous_year: this.jockey_shows_previous_year,
      jockey_roi_previous_year: this.jockey_roi_previous_year,
      sire_stud_fee: this.sire_stud_fee,
      best_bris_speed_fast_track: this.best_bris_speed_fast_track,
      best_bris_speed_turf: this.best_bris_speed_turf,
      best_bris_speed_off_track: this.best_bris_speed_off_track,
      best_bris_speed_distance: this.best_bris_speed_distance,
      auction_price: this.auction_price,
      where_when_sold_at_auction: this.where_when_sold_at_auction,
      bris_dirt_pedigree_rating: this.bris_dirt_pedigree_rating,
      bris_mud_pedigree_rating: this.bris_mud_pedigree_rating,
      bris_turf_pedigree_rating: this.bris_turf_pedigree_rating,
      bris_distance_pedigree_rating: this.bris_distance_pedigree_rating,
      best_bris_speed_life: this.best_bris_speed_life,
      best_bris_speed_most_recent_year: this.best_bris_speed_most_recent_year,
      best_bris_speed_2nd_most_recent_year: this.best_bris_speed_2nd_most_recent_year,
      best_bris_speed_todays_track: this.best_bris_speed_todays_track,
      starts_fast_dirt: this.starts_fast_dirt,
      wins_fast_dirt: this.wins_fast_dirt,
      places_fast_dirt: this.places_fast_dirt,
      shows_fast_dirt: this.shows_fast_dirt,
      earnings_fast_dirt: this.earnings_fast_dirt,
      jockey_distance_turf_label: this.jockey_distance_turf_label,
      jockey_distance_turf_starts: this.jockey_distance_turf_starts,
      jockey_distance_turf_wins: this.jockey_distance_turf_wins,
      jockey_distance_turf_places: this.jockey_distance_turf_places,
      jockey_distance_turf_shows: this.jockey_distance_turf_shows,
      jockey_distance_turf_roi: this.jockey_distance_turf_roi,
      jockey_distance_turf_earnings: this.jockey_distance_turf_earnings,
      trainer_jockey_combo_starts_meet: this.trainer_jockey_combo_starts_meet,
      trainer_jockey_combo_wins_meet: this.trainer_jockey_combo_wins_meet,
      trainer_jockey_combo_places_meet: this.trainer_jockey_combo_places_meet,
      trainer_jockey_combo_shows_meet: this.trainer_jockey_combo_shows_meet,
      trainer_jockey_combo_roi_meet: this.trainer_jockey_combo_roi_meet,
      note: this.note,
      workouts: this.workouts.map(w => w.toObject()),
      past_performances: this.past_performances.map(p => p.toObject()),
      key_trainer_stats: this.key_trainer_stats.map(k => k.toObject())
    };
  }
}

export class Workout {
  id: number;
  horse_id: number;
  date: string;
  time: number | null;
  track: string;
  distance: number | null;
  condition: string;
  description: string;
  main_inner_track_indicator: string;
  workouts_that_day_distance: number | null;
  rank: number | null;

  constructor(data: any = {}) {
    this.id = data.id ?? 0;
    this.horse_id = data.horse_id ?? 0;
    this.date = data.date ?? '';
    this.time = data.time ?? null;
    this.track = data.track ?? '';
    this.distance = data.distance ?? null;
    this.condition = data.condition ?? '';
    this.description = data.description ?? '';
    this.main_inner_track_indicator = data.main_inner_track_indicator ?? '';
    this.workouts_that_day_distance = data.workouts_that_day_distance ?? null;
    this.rank = data.rank ?? null;
  }

  static fromObject(obj: any): Workout {
    return new Workout(obj);
  }

  toObject(): any {
    return {
      id: this.id,
      horse_id: this.horse_id,
      date: this.date,
      time: this.time,
      track: this.track,
      distance: this.distance,
      condition: this.condition,
      description: this.description,
      main_inner_track_indicator: this.main_inner_track_indicator,
      workouts_that_day_distance: this.workouts_that_day_distance,
      rank: this.rank
    };
  }
}

export class KeyTrainerStat {
  id: number;
  horse_id: number;
  category: string;
  starts: number | null;
  win_pct: number | null;
  in_the_money_pct: number | null;
  roi: number | null;

  constructor(data: any = {}) {
    this.id = data.id ?? 0;
    this.horse_id = data.horse_id ?? 0;
    this.category = data.category ?? '';
    this.starts = data.starts ?? null;
    this.win_pct = data.win_pct ?? null;
    this.in_the_money_pct = data.in_the_money_pct ?? null;
    this.roi = data.roi ?? null;
  }

  static fromObject(obj: any): KeyTrainerStat {
    return new KeyTrainerStat({
      id: obj.id,
      horse_id: obj.horse_id,
      category: obj.category,
      starts: obj.starts,
      win_pct: obj.win_pct,
      in_the_money_pct: obj.in_the_money_pct,
      roi: obj.roi
    });
  }

  toObject(): any {
    return {
      id: this.id,
      horse_id: this.horse_id,
      category: this.category,
      starts: this.starts,
      win_pct: this.win_pct,
      in_the_money_pct: this.in_the_money_pct,
      roi: this.roi
    };
  }
}

export class PastPerformance {
  id: number;
  horse_id: number;
  race_date: string;
  days_since_last_race: number | null;
  track_code: string;
  bris_track_code: string;
  race_number: number | null;
  track_condition: string;
  distance: number | null;
  surface: string;
  special_chute_indicator: string;
  entrants: number | null;
  post_position: number | null;
  equipment: string;
  racename: string;
  medication: number | null;
  trip_comment: string;
  winners_name: string;
  place_name: string;
  show_name: string;
  winners_weight: number | null;
  place_weight: number | null;
  show_weight: number | null;
  winners_margin: number | null;
  place_margin: number | null;
  show_margin: number | null;
  alternate_comment_line: string;
  weight: number | null;
  odds: number | null;
  entry: string;
  race_classication: string;
  claiming_price: number | null;
  purse: number | null;
  start_call_position: string;
  first_call_position: string;
  second_call_position: string;
  gate_call_position: string;
  stretch_call_position: string;
  finish_position: string;
  money_position: string;
  start_call_between_lengths_leader: number | null;
  start_call_between_lengths: number | null;
  first_call_between_lengths_leader: number | null;
  first_call_between_lengths: number | null;
  second_call_between_lengths_leader: number | null;
  second_call_between_lengths: number | null;
  bris_race_shape_1st_call: number | null;
  stretch_call_between_lengths_leader: number | null;
  stretch_call_between_lengths: number | null;
  finish_between_lengths_leader: number | null;
  finish_between_lengths: number | null;
  bris_race_shape_2nd_call: number | null;
  bris_2f_pace: number | null;
  bris_4f_pace: number | null;
  bris_6f_pace: number | null;
  bris_8f_pace: number | null;
  bris_10f_pace: number | null;
  bris_late_pace: number | null;
  bris_speed_rating: number | null;
  speed_rating: number | null;
  track_variant: number | null;
  two_f_fraction: number | null;
  three_f_fraction: number | null;
  four_f_fraction: number | null;
  five_f_fraction: number | null;
  six_f_fraction: number | null;
  seven_f_fraction: number | null;
  eight_f_fraction: number | null;
  ten_f_fraction: number | null;
  twelve_f_fraction: number | null;
  fourteen_f_fraction: number | null;
  sixteen_f_fraction: number | null;
  fraction_1: number | null;
  fraction_2: number | null;
  fraction_3: number | null;
  final_time: number | null;
  claimed_code: string;
  trainer: string;
  jockey: string;
  apprentice_weight_allowance: number | null;
  race_type: string;
  age_sex_restrictions: string;
  statebred_flag: string;
  restricted_qualifier_flag: string;
  favorite_indicator: string;
  front_bandages_indicator: string;
  bris_speed_par_for_race: number | null;
  bar_shoes: string;
  company_line_codes: string;
  low_claiming_price_of_race: number | null;
  high_claiming_price_of_race: number | null;
  code_for_prior_races: string;
  claimed_and_trainer_switches_1: string;
  claimed_and_trainer_switches_2: string;
  claimed_and_trainer_switches_3: string;
  claimed_and_trainer_switches_4: string;
  claimed_and_trainer_switches_5: string;
  claimed_and_trainer_switches_6: string;
  extended_start_comment: string;
  sealed_track_indicator: string;
  previous_all_weather_surface_indicator: string;
  equibase_abbreviated_race_condition: string;

  constructor(data: any = {}) {
    this.id = data.id ?? 0;
    this.horse_id = data.horse_id ?? 0;
    this.race_date = data.race_date ?? '';
    this.days_since_last_race = data.days_since_last_race ?? null;
    this.track_code = data.track_code ?? '';
    this.bris_track_code = data.bris_track_code ?? '';
    this.race_number = data.race_number ?? null;
    this.track_condition = data.track_condition ?? '';
    this.distance = data.distance ?? null;
    this.surface = data.surface ?? '';
    this.special_chute_indicator = data.special_chute_indicator ?? '';
    this.entrants = data.entrants ?? null;
    this.post_position = data.post_position ?? null;
    this.equipment = data.equipment ?? '';
    this.racename = data.racename ?? '';
    this.medication = data.medication ?? null;
    this.trip_comment = data.trip_comment ?? '';
    this.winners_name = data.winners_name ?? '';
    this.place_name = data.place_name ?? '';
    this.show_name = data.show_name ?? '';
    this.winners_weight = data.winners_weight ?? null;
    this.place_weight = data.place_weight ?? null;
    this.show_weight = data.show_weight ?? null;
    this.winners_margin = data.winners_margin ?? null;
    this.place_margin = data.place_margin ?? null;
    this.show_margin = data.show_margin ?? null;
    this.alternate_comment_line = data.alternate_comment_line ?? '';
    this.weight = data.weight ?? null;
    this.odds = data.odds ?? null;
    this.entry = data.entry ?? '';
    this.race_classication = data.race_classication ?? '';
    this.claiming_price = data.claiming_price ?? null;
    this.purse = data.purse ?? null;
    this.start_call_position = data.start_call_position ?? '';
    this.first_call_position = data.first_call_position ?? '';
    this.second_call_position = data.second_call_position ?? '';
    this.gate_call_position = data.gate_call_position ?? '';
    this.stretch_call_position = data.stretch_call_position ?? '';
    this.finish_position = data.finish_position ?? '';
    this.money_position = data.money_position ?? '';
    this.start_call_between_lengths_leader = data.start_call_between_lengths_leader ?? null;
    this.start_call_between_lengths = data.start_call_between_lengths ?? null;
    this.first_call_between_lengths_leader = data.first_call_between_lengths_leader ?? null;
    this.first_call_between_lengths = data.first_call_between_lengths ?? null;
    this.second_call_between_lengths_leader = data.second_call_between_lengths_leader ?? null;
    this.second_call_between_lengths = data.second_call_between_lengths ?? null;
    this.bris_race_shape_1st_call = data.bris_race_shape_1st_call ?? null;
    this.stretch_call_between_lengths_leader = data.stretch_call_between_lengths_leader ?? null;
    this.stretch_call_between_lengths = data.stretch_call_between_lengths ?? null;
    this.finish_between_lengths_leader = data.finish_between_lengths_leader ?? null;
    this.finish_between_lengths = data.finish_between_lengths ?? null;
    this.bris_race_shape_2nd_call = data.bris_race_shape_2nd_call ?? null;
    this.bris_2f_pace = data.bris_2f_pace ?? null;
    this.bris_4f_pace = data.bris_4f_pace ?? null;
    this.bris_6f_pace = data.bris_6f_pace ?? null;
    this.bris_8f_pace = data.bris_8f_pace ?? null;
    this.bris_10f_pace = data.bris_10f_pace ?? null;
    this.bris_late_pace = data.bris_late_pace ?? null;
    this.bris_speed_rating = data.bris_speed_rating ?? null;
    this.speed_rating = data.speed_rating ?? null;
    this.track_variant = data.track_variant ?? null;
    this.two_f_fraction = data.two_f_fraction ?? null;
    this.three_f_fraction = data.three_f_fraction ?? null;
    this.four_f_fraction = data.four_f_fraction ?? null;
    this.five_f_fraction = data.five_f_fraction ?? null;
    this.six_f_fraction = data.six_f_fraction ?? null;
    this.seven_f_fraction = data.seven_f_fraction ?? null;
    this.eight_f_fraction = data.eight_f_fraction ?? null;
    this.ten_f_fraction = data.ten_f_fraction ?? null;
    this.twelve_f_fraction = data.twelve_f_fraction ?? null;
    this.fourteen_f_fraction = data.fourteen_f_fraction ?? null;
    this.sixteen_f_fraction = data.sixteen_f_fraction ?? null;
    this.fraction_1 = data.fraction_1 ?? null;
    this.fraction_2 = data.fraction_2 ?? null;
    this.fraction_3 = data.fraction_3 ?? null;
    this.final_time = data.final_time ?? null;
    this.claimed_code = data.claimed_code ?? '';
    this.trainer = data.trainer ?? '';
    this.jockey = data.jockey ?? '';
    this.apprentice_weight_allowance = data.apprentice_weight_allowance ?? null;
    this.race_type = data.race_type ?? '';
    this.age_sex_restrictions = data.age_sex_restrictions ?? '';
    this.statebred_flag = data.statebred_flag ?? '';
    this.restricted_qualifier_flag = data.restricted_qualifier_flag ?? '';
    this.favorite_indicator = data.favorite_indicator ?? '';
    this.front_bandages_indicator = data.front_bandages_indicator ?? '';
    this.bris_speed_par_for_race = data.bris_speed_par_for_race ?? null;
    this.bar_shoes = data.bar_shoes ?? '';
    this.company_line_codes = data.company_line_codes ?? '';
    this.low_claiming_price_of_race = data.low_claiming_price_of_race ?? null;
    this.high_claiming_price_of_race = data.high_claiming_price_of_race ?? null;
    this.code_for_prior_races = data.code_for_prior_races ?? '';
    this.claimed_and_trainer_switches_1 = data.claimed_and_trainer_switches_1 ?? '';
    this.claimed_and_trainer_switches_2 = data.claimed_and_trainer_switches_2 ?? '';
    this.claimed_and_trainer_switches_3 = data.claimed_and_trainer_switches_3 ?? '';
    this.claimed_and_trainer_switches_4 = data.claimed_and_trainer_switches_4 ?? '';
    this.claimed_and_trainer_switches_5 = data.claimed_and_trainer_switches_5 ?? '';
    this.claimed_and_trainer_switches_6 = data.claimed_and_trainer_switches_6 ?? '';
    this.extended_start_comment = data.extended_start_comment ?? '';
    this.sealed_track_indicator = data.sealed_track_indicator ?? '';
    this.previous_all_weather_surface_indicator = data.previous_all_weather_surface_indicator ?? '';
    this.equibase_abbreviated_race_condition = data.equibase_abbreviated_race_condition ?? '';
  }

  static fromObject(obj: any): PastPerformance {
    return new PastPerformance(obj);
  }

  toObject(): any {
    return {
      id: this.id,
      horse_id: this.horse_id,
      race_date: this.race_date,
      days_since_last_race: this.days_since_last_race,
      track_code: this.track_code,
      bris_track_code: this.bris_track_code,
      race_number: this.race_number,
      track_condition: this.track_condition,
      distance: this.distance,
      surface: this.surface,
      special_chute_indicator: this.special_chute_indicator,
      entrants: this.entrants,
      post_position: this.post_position,
      equipment: this.equipment,
      racename: this.racename,
      medication: this.medication,
      trip_comment: this.trip_comment,
      winners_name: this.winners_name,
      place_name: this.place_name,
      show_name: this.show_name,
      winners_weight: this.winners_weight,
      place_weight: this.place_weight,
      show_weight: this.show_weight,
      winners_margin: this.winners_margin,
      place_margin: this.place_margin,
      show_margin: this.show_margin,
      alternate_comment_line: this.alternate_comment_line,
      weight: this.weight,
      odds: this.odds,
      entry: this.entry,
      race_classication: this.race_classication,
      claiming_price: this.claiming_price,
      purse: this.purse,
      start_call_position: this.start_call_position,
      first_call_position: this.first_call_position,
      second_call_position: this.second_call_position,
      gate_call_position: this.gate_call_position,
      stretch_call_position: this.stretch_call_position,
      finish_position: this.finish_position,
      money_position: this.money_position,
      start_call_between_lengths_leader: this.start_call_between_lengths_leader,
      start_call_between_lengths: this.start_call_between_lengths,
      first_call_between_lengths_leader: this.first_call_between_lengths_leader,
      first_call_between_lengths: this.first_call_between_lengths,
      second_call_between_lengths_leader: this.second_call_between_lengths_leader,
      second_call_between_lengths: this.second_call_between_lengths,
      bris_race_shape_1st_call: this.bris_race_shape_1st_call,
      stretch_call_between_lengths_leader: this.stretch_call_between_lengths_leader,
      stretch_call_between_lengths: this.stretch_call_between_lengths,
      finish_between_lengths_leader: this.finish_between_lengths_leader,
      finish_between_lengths: this.finish_between_lengths,
      bris_race_shape_2nd_call: this.bris_race_shape_2nd_call,
      bris_2f_pace: this.bris_2f_pace,
      bris_4f_pace: this.bris_4f_pace,
      bris_6f_pace: this.bris_6f_pace,
      bris_8f_pace: this.bris_8f_pace,
      bris_10f_pace: this.bris_10f_pace,
      bris_late_pace: this.bris_late_pace,
      bris_speed_rating: this.bris_speed_rating,
      speed_rating: this.speed_rating,
      track_variant: this.track_variant,
      two_f_fraction: this.two_f_fraction,
      three_f_fraction: this.three_f_fraction,
      four_f_fraction: this.four_f_fraction,
      five_f_fraction: this.five_f_fraction,
      six_f_fraction: this.six_f_fraction,
      seven_f_fraction: this.seven_f_fraction,
      eight_f_fraction: this.eight_f_fraction,
      ten_f_fraction: this.ten_f_fraction,
      twelve_f_fraction: this.twelve_f_fraction,
      fourteen_f_fraction: this.fourteen_f_fraction,
      sixteen_f_fraction: this.sixteen_f_fraction,
      fraction_1: this.fraction_1,
      fraction_2: this.fraction_2,
      fraction_3: this.fraction_3,
      final_time: this.final_time,
      claimed_code: this.claimed_code,
      trainer: this.trainer,
      jockey: this.jockey,
      apprentice_weight_allowance: this.apprentice_weight_allowance,
      race_type: this.race_type,
      age_sex_restrictions: this.age_sex_restrictions,
      statebred_flag: this.statebred_flag,
      restricted_qualifier_flag: this.restricted_qualifier_flag,
      favorite_indicator: this.favorite_indicator,
      front_bandages_indicator: this.front_bandages_indicator,
      bris_speed_par_for_race: this.bris_speed_par_for_race,
      bar_shoes: this.bar_shoes,
      company_line_codes: this.company_line_codes,
      low_claiming_price_of_race: this.low_claiming_price_of_race,
      high_claiming_price_of_race: this.high_claiming_price_of_race,
      code_for_prior_races: this.code_for_prior_races,
      claimed_and_trainer_switches_1: this.claimed_and_trainer_switches_1,
      claimed_and_trainer_switches_2: this.claimed_and_trainer_switches_2,
      claimed_and_trainer_switches_3: this.claimed_and_trainer_switches_3,
      claimed_and_trainer_switches_4: this.claimed_and_trainer_switches_4,
      claimed_and_trainer_switches_5: this.claimed_and_trainer_switches_5,
      claimed_and_trainer_switches_6: this.claimed_and_trainer_switches_6,
      extended_start_comment: this.extended_start_comment,
      sealed_track_indicator: this.sealed_track_indicator,
      previous_all_weather_surface_indicator: this.previous_all_weather_surface_indicator,
      equibase_abbreviated_race_condition: this.equibase_abbreviated_race_condition
    };
  }
}
