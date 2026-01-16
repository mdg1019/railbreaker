export class Racecard {
  track: string;
  date: string;
  longDate: string;
  races: Race[];

  constructor(data: any = {}) {
    this.track = data.track ?? '';
    this.date = data.date ?? '';
    this.longDate = data.longDate ?? '';
    this.races = data.races ?? [];
  }

  static fromObject(obj: any): Racecard {
    return new Racecard({
      track: obj.track,
      date: obj.date,
      longDate: obj.longDate,
      races: obj.races?.map((r: any) => Race.fromObject(r)) ?? []
    });
  }

  toObject(): any {
    return {
      track: this.track,
      date: this.date,
      longDate: this.longDate,
      races: this.races.map(r => r.toObject())
    };
  }
}

export class Race {
  raceNumber: number | null;
  distance: number | null;
  surface: string;
  raceType: string;
  ageSexRestrictions: string;
  todaysRaceClassification: string;
  purse: number | null;
  claimingPrice: number | null;
  trackRecord: number | null;
  raceConditions: string;
  todaysLasixList: string;
  todaysButeList: string;
  todaysCoupledList: string;
  todaysMutuelList: string;
  simulcastHostTrackCode: string;
  simulcastHostTrackRaceNumber: number | null;
  allWeatherSurfaceFlag: string;
  raceConditionsLine1: string;
  raceConditionsLine2: string;
  raceConditionsLine3: string;
  raceConditionsLine4: string;
  raceConditionsLine5: string;
  raceConditionsLine6: string;
  lowClaimingPrice: number | null;
  statebredFlag: string;
  wagerTypeLine1: string;
  wagerTypeLine2: string;
  wagerTypeLine3: string;
  wagerTypeLine4: string;
  wagerTypeLine5: string;
  wagerTypeLine6: string;
  wagerTypeLine7: string;
  wagerTypeLine8: string;
  wagerTypeLine9: string;
  twoFBrisPacePar: number | null;
  fourFBrisPacePar: number | null;
  sixFBrisPacePar: number | null;
  brisSpeedForClass: number | null;
  postTimes: string;
  postTimePacificMilitary: string;
  brisLatePacePar: number | null;
  todaysEquibaseAbbreviatedRaceConditions: string;
  horses: Horse[];

  constructor(data: any = {}) {
    this.raceNumber = data.raceNumber ?? null;
    this.distance = data.distance ?? null;
    this.surface = data.surface ?? '';
    this.raceType = data.raceType ?? '';
    this.ageSexRestrictions = data.ageSexRestrictions ?? '';
    this.todaysRaceClassification = data.todaysRaceClassification ?? '';
    this.purse = data.purse ?? null;
    this.claimingPrice = data.claimingPrice ?? null;
    this.trackRecord = data.trackRecord ?? null;
    this.raceConditions = data.raceConditions ?? '';
    this.todaysLasixList = data.todaysLasixList ?? '';
    this.todaysButeList = data.todaysButeList ?? '';
    this.todaysCoupledList = data.todaysCoupledList ?? '';
    this.todaysMutuelList = data.todaysMutuelList ?? '';
    this.simulcastHostTrackCode = data.simulcastHostTrackCode ?? '';
    this.simulcastHostTrackRaceNumber = data.simulcastHostTrackRaceNumber ?? null;
    this.allWeatherSurfaceFlag = data.allWeatherSurfaceFlag ?? '';
    this.raceConditionsLine1 = data.raceConditionsLine1 ?? '';
    this.raceConditionsLine2 = data.raceConditionsLine2 ?? '';
    this.raceConditionsLine3 = data.raceConditionsLine3 ?? '';
    this.raceConditionsLine4 = data.raceConditionsLine4 ?? '';
    this.raceConditionsLine5 = data.raceConditionsLine5 ?? '';
    this.raceConditionsLine6 = data.raceConditionsLine6 ?? '';
    this.lowClaimingPrice = data.lowClaimingPrice ?? null;
    this.statebredFlag = data.statebredFlag ?? '';
    this.wagerTypeLine1 = data.wagerTypeLine1 ?? '';
    this.wagerTypeLine2 = data.wagerTypeLine2 ?? '';
    this.wagerTypeLine3 = data.wagerTypeLine3 ?? '';
    this.wagerTypeLine4 = data.wagerTypeLine4 ?? '';
    this.wagerTypeLine5 = data.wagerTypeLine5 ?? '';
    this.wagerTypeLine6 = data.wagerTypeLine6 ?? '';
    this.wagerTypeLine7 = data.wagerTypeLine7 ?? '';
    this.wagerTypeLine8 = data.wagerTypeLine8 ?? '';
    this.wagerTypeLine9 = data.wagerTypeLine9 ?? '';
    this.twoFBrisPacePar = data.twoFBrisPacePar ?? null;
    this.fourFBrisPacePar = data.fourFBrisPacePar ?? null;
    this.sixFBrisPacePar = data.sixFBrisPacePar ?? null;
    this.brisSpeedForClass = data.brisSpeedForClass ?? null;
    this.brisLatePacePar = data.brisLatePacePar ?? null;
    this.postTimes = data.postTimes ?? '';
    this.postTimePacificMilitary = data.postTimePacificMilitary ?? '';
    this.todaysEquibaseAbbreviatedRaceConditions = data.todaysEquibaseAbbreviatedRaceConditions ?? '';
    this.horses = data.horses ?? [];
  }

  static fromObject(obj: any): Race {
    return new Race({
      ...obj,
      horses: obj.horses?.map((h: any) => Horse.fromObject(h)) ?? []
    });
  }

  toObject(): any {
    return {
      raceNumber: this.raceNumber,
      distance: this.distance,
      surface: this.surface,
      raceType: this.raceType,
      ageSexRestrictions: this.ageSexRestrictions,
      todaysRaceClassification: this.todaysRaceClassification,
      purse: this.purse,
      claimingPrice: this.claimingPrice,
      trackRecord: this.trackRecord,
      raceConditions: this.raceConditions,
      todaysLasixList: this.todaysLasixList,
      todaysButeList: this.todaysButeList,
      todaysCoupledList: this.todaysCoupledList,
      todaysMutuelList: this.todaysMutuelList,
      simulcastHostTrackCode: this.simulcastHostTrackCode,
      simulcastHostTrackRaceNumber: this.simulcastHostTrackRaceNumber,
      allWeatherSurfaceFlag: this.allWeatherSurfaceFlag,
      raceConditionsLine1: this.raceConditionsLine1,
      raceConditionsLine2: this.raceConditionsLine2,
      raceConditionsLine3: this.raceConditionsLine3,
      raceConditionsLine4: this.raceConditionsLine4,
      raceConditionsLine5: this.raceConditionsLine5,
      raceConditionsLine6: this.raceConditionsLine6,
      lowClaimingPrice: this.lowClaimingPrice,
      statebredFlag: this.statebredFlag,
      wagerTypeLine1: this.wagerTypeLine1,
      wagerTypeLine2: this.wagerTypeLine2,
      wagerTypeLine3: this.wagerTypeLine3,
      wagerTypeLine4: this.wagerTypeLine4,
      wagerTypeLine5: this.wagerTypeLine5,
      wagerTypeLine6: this.wagerTypeLine6,
      wagerTypeLine7: this.wagerTypeLine7,
      wagerTypeLine8: this.wagerTypeLine8,
      wagerTypeLine9: this.wagerTypeLine9,
      twoFBrisPacePar: this.twoFBrisPacePar,
      fourFBrisPacePar: this.fourFBrisPacePar,
      sixFBrisPacePar: this.sixFBrisPacePar,
      brisSpeedForClass: this.brisSpeedForClass,
      brisLatePacePar: this.brisLatePacePar,
      postTimes: this.postTimes,
      postTimePacificMilitary: this.postTimePacificMilitary,
      todaysEquibaseAbbreviatedRaceConditions: this.todaysEquibaseAbbreviatedRaceConditions,
      horses: this.horses.map(h => h.toObject())
    };
  }
}

export class Horse {
  postPosition: number | null;
  entry: string;
  claimingPriceOfHorse: number | null;
  breedType: string;
  todaysNasalStripChange: number | null;
  todaysTrainer: string;
  trainerStarts: number | null;
  trainerWins: number | null;
  trainerPlaces: number | null;
  trainerShows: number | null;
  todaysJockey: string;
  apprenticeWeightAllowance: number | null;
  jockeyStarts: number | null;
  jockeyWins: number | null;
  jockeyPlaces: number | null;
  jockeyShows: number | null;
  todaysOwner: string;
  ownersSilks: string;
  mainTrackOnlyAeIndicator: string;
  programNumber: string;
  morningLineOdds: number | null;
  horseName: string;
  yearOfBirth: number | null;
  horsesFoalingMonth: number | null;
  sex: string;
  horsesColor: string;
  weight: number | null;
  sire: string;
  siresSire: string;
  dam: string;
  damsSire: string;
  breeder: string;
  stateCountryWhereBred: string;
  programPostPosition: string;
  todaysMedicationNew: number | null;
  todaysMedicationOld: number | null;
  equipmentChange: number | null;
  lifetimeRecordTodaysDistanceStarts: number | null;
  lifetimeRecordTodaysDistanceWins: number | null;
  lifetimeRecordTodaysDistancePlaces: number | null;
  lifetimeRecordTodaysDistanceShows: number | null;
  lifetimeRecordTodaysDistanceEarnings: number | null;
  lifetimeRecordTodaysTrackStarts: number | null;
  lifetimeRecordTodaysTrackWins: number | null;
  lifetimeRecordTodaysTrackPlaces: number | null;
  lifetimeRecordTodaysTrackShows: number | null;
  lifetimeRecordTodaysTrackEarnings: number | null;
  lifetimeRecordTurfStarts: number | null;
  lifetimeRecordTurfWins: number | null;
  lifetimeRecordTurfPlaces: number | null;
  lifetimeRecordTurfShows: number | null;
  lifetimeRecordTurfEarnings: number | null;
  lifetimeRecordWetStarts: number | null;
  lifetimeRecordWetWins: number | null;
  lifetimeRecordWetPlaces: number | null;
  lifetimeRecordWetShows: number | null;
  lifetimeRecordWetEarnings: number | null;
  currentYearRecordYear: number | null;
  currentYearRecordStarts: number | null;
  currentYearRecordWins: number | null;
  currentYearRecordPlaces: number | null;
  currentYearRecordShows: number | null;
  currentYearRecordEarnings: number | null;
  previousYearRecordYear: number | null;
  previousYearRecordStarts: number | null;
  previousYearRecordWins: number | null;
  previousYearRecordPlaces: number | null;
  previousYearRecordShows: number | null;
  previousYearRecordEarnings: number | null;
  lifetimeRecordStarts: number | null;
  lifetimeRecordWins: number | null;
  lifetimeRecordPlaces: number | null;
  lifetimeRecordShows: number | null;
  lifetimeRecordEarnings: number | null;
  brisRunStyle: string;
  quirinSpeedPoints: number | null;
  trainerJockeyComboStarts: number | null;
  trainerJockeyComboWins: number | null;
  trainerJockeyComboPlaces: number | null;
  trainerJockeyComboShows: number | null;
  trainerJockeyComboRoi: number | null;
  daysSinceLastRace: number | null;
  lifetimeAllWeatherStarts: number | null;
  lifetimeAllWeatherWins: number | null;
  lifetimeAllWeatherPlaces: number | null;
  lifetimeAllWeatherShows: number | null;
  lifetimeAllWeatherEarnings: number | null;
  bestBrisSpeedAllWeatherSurface: number | null;
  brisPrimePowerRating: number | null;
  trainerStartsCurrentYear: number | null;
  trainerWinsCurrentYear: number | null;
  trainerPlacesCurrentYear: number | null;
  trainerShowsCurrentYear: number | null;
  trainerRoiCurrentYear: number | null;
  trainerStartsPreviousYear: number | null;
  trainerWinsPreviousYear: number | null;
  trainerPlacesPreviousYear: number | null;
  trainerShowsPreviousYear: number | null;
  trainerRoiPreviousYear: number | null;
  jockeyStartsCurrentYear: number | null;
  jockeyWinsCurrentYear: number | null;
  jockeyPlacesCurrentYear: number | null;
  jockeyShowsCurrentYear: number | null;
  jockeyRoiCurrentYear: number | null;
  jockeyStartsPreviousYear: number | null;
  jockeyWinsPreviousYear: number | null;
  jockeyPlacesPreviousYear: number | null;
  jockeyShowsPreviousYear: number | null;
  jockeyRoiPreviousYear: number | null;
  sireStudFee: number | null;
  bestBrisSpeedFastTrack: number | null;
  bestBrisSpeedTurf: number | null;
  bestBrisSpeedOffTrack: number | null;
  bestBrisSpeedDistance: number | null;
  auctionPrice: number | null;
  whereWhenSoldAtAuction: string;
  brisDirtPedigreeRating: string;
  brisMudPedigreeRating: string;
  brisTurfPedigreeRating: string;
  brisDistancePedigreeRating: string;
  bestBrisSpeedLife: number | null;
  bestBrisSpeedMostRecentYear: number | null;
  bestBrisSpeed2ndMostRecentYear: number | null;
  bestBrisSpeedTodaysTrack: number | null;
  startsFastDirt: number | null;
  winsFastDirt: number | null;
  placesFastDirt: number | null;
  showsFastDirt: number | null;
  earningsFastDirt: number | null;
  jockeyDistanceTurfLabel: string;
  jockeyDistanceTurfStarts: number | null;
  jockeyDistanceTurfWins: number | null;
  jockeyDistanceTurfPlaces: number | null;
  jockeyDistanceTurfShows: number | null;
  jockeyDistanceTurfRoi: number | null;
  jockeyDistanceTurfEarnings: number | null;
  trainerJockeyComboStartsMeet: number | null;
  trainerJockeyComboWinsMeet: number | null;
  trainerJockeyComboPlacesMeet: number | null;
  trainerJockeyComboShowsMeet: number | null;
  trainerJockeyComboRoiMeet: number | null;
  workouts: Workout[];
  pastPerformances: PastPerformance[];
  keyTrainerStats: KeyTrainerStat[];

  constructor(data: any = {}) {
    this.postPosition = data.postPosition ?? null;
    this.entry = data.entry ?? '';
    this.claimingPriceOfHorse = data.claimingPriceOfHorse ?? null;
    this.breedType = data.breedType ?? '';
    this.todaysNasalStripChange = data.todaysNasalStripChange ?? null;
    this.todaysTrainer = data.todaysTrainer ?? '';
    this.trainerStarts = data.trainerStarts ?? null;
    this.trainerWins = data.trainerWins ?? null;
    this.trainerPlaces = data.trainerPlaces ?? null;
    this.trainerShows = data.trainerShows ?? null;
    this.todaysJockey = data.todaysJockey ?? '';
    this.apprenticeWeightAllowance = data.apprenticeWeightAllowance ?? null;
    this.jockeyStarts = data.jockeyStarts ?? null;
    this.jockeyWins = data.jockeyWins ?? null;
    this.jockeyPlaces = data.jockeyPlaces ?? null;
    this.jockeyShows = data.jockeyShows ?? null;
    this.todaysOwner = data.todaysOwner ?? '';
    this.ownersSilks = data.ownersSilks ?? '';
    this.mainTrackOnlyAeIndicator = data.mainTrackOnlyAeIndicator ?? '';
    this.programNumber = data.programNumber ?? '';
    this.morningLineOdds = data.morningLineOdds ?? null;
    this.horseName = data.horseName ?? '';
    this.yearOfBirth = data.yearOfBirth ?? null;
    this.horsesFoalingMonth = data.horsesFoalingMonth ?? null;
    this.sex = data.sex ?? '';
    this.horsesColor = data.horsesColor ?? '';
    this.weight = data.weight ?? null;
    this.sire = data.sire ?? '';
    this.siresSire = data.siresSire ?? '';
    this.dam = data.dam ?? '';
    this.damsSire = data.damsSire ?? '';
    this.breeder = data.breeder ?? '';
    this.stateCountryWhereBred = data.stateCountryWhereBred ?? '';
    this.programPostPosition = data.programPostPosition ?? '';
    this.todaysMedicationNew = data.todaysMedicationNew ?? null;
    this.todaysMedicationOld = data.todaysMedicationOld ?? null;
    this.equipmentChange = data.equipmentChange ?? null;
    this.lifetimeRecordTodaysDistanceStarts = data.lifetimeRecordTodaysDistanceStarts ?? null;
    this.lifetimeRecordTodaysDistanceWins = data.lifetimeRecordTodaysDistanceWins ?? null;
    this.lifetimeRecordTodaysDistancePlaces = data.lifetimeRecordTodaysDistancePlaces ?? null;
    this.lifetimeRecordTodaysDistanceShows = data.lifetimeRecordTodaysDistanceShows ?? null;
    this.lifetimeRecordTodaysDistanceEarnings = data.lifetimeRecordTodaysDistanceEarnings ?? null;
    this.lifetimeRecordTodaysTrackStarts = data.lifetimeRecordTodaysTrackStarts ?? null;
    this.lifetimeRecordTodaysTrackWins = data.lifetimeRecordTodaysTrackWins ?? null;
    this.lifetimeRecordTodaysTrackPlaces = data.lifetimeRecordTodaysTrackPlaces ?? null;
    this.lifetimeRecordTodaysTrackShows = data.lifetimeRecordTodaysTrackShows ?? null;
    this.lifetimeRecordTodaysTrackEarnings = data.lifetimeRecordTodaysTrackEarnings ?? null;
    this.lifetimeRecordTurfStarts = data.lifetimeRecordTurfStarts ?? null;
    this.lifetimeRecordTurfWins = data.lifetimeRecordTurfWins ?? null;
    this.lifetimeRecordTurfPlaces = data.lifetimeRecordTurfPlaces ?? null;
    this.lifetimeRecordTurfShows = data.lifetimeRecordTurfShows ?? null;
    this.lifetimeRecordTurfEarnings = data.lifetimeRecordTurfEarnings ?? null;
    this.lifetimeRecordWetStarts = data.lifetimeRecordWetStarts ?? null;
    this.lifetimeRecordWetWins = data.lifetimeRecordWetWins ?? null;
    this.lifetimeRecordWetPlaces = data.lifetimeRecordWetPlaces ?? null;
    this.lifetimeRecordWetShows = data.lifetimeRecordWetShows ?? null;
    this.lifetimeRecordWetEarnings = data.lifetimeRecordWetEarnings ?? null;
    this.currentYearRecordYear = data.currentYearRecordYear ?? null;
    this.currentYearRecordStarts = data.currentYearRecordStarts ?? null;
    this.currentYearRecordWins = data.currentYearRecordWins ?? null;
    this.currentYearRecordPlaces = data.currentYearRecordPlaces ?? null;
    this.currentYearRecordShows = data.currentYearRecordShows ?? null;
    this.currentYearRecordEarnings = data.currentYearRecordEarnings ?? null;
    this.previousYearRecordYear = data.previousYearRecordYear ?? null;
    this.previousYearRecordStarts = data.previousYearRecordStarts ?? null;
    this.previousYearRecordWins = data.previousYearRecordWins ?? null;
    this.previousYearRecordPlaces = data.previousYearRecordPlaces ?? null;
    this.previousYearRecordShows = data.previousYearRecordShows ?? null;
    this.previousYearRecordEarnings = data.previousYearRecordEarnings ?? null;
    this.lifetimeRecordStarts = data.lifetimeRecordStarts ?? null;
    this.lifetimeRecordWins = data.lifetimeRecordWins ?? null;
    this.lifetimeRecordPlaces = data.lifetimeRecordPlaces ?? null;
    this.lifetimeRecordShows = data.lifetimeRecordShows ?? null;
    this.lifetimeRecordEarnings = data.lifetimeRecordEarnings ?? null;
    this.brisRunStyle = data.brisRunStyle ?? '';
    this.quirinSpeedPoints = data.quirinSpeedPoints ?? null;
    this.trainerJockeyComboStarts = data.trainerJockeyComboStarts ?? null;
    this.trainerJockeyComboWins = data.trainerJockeyComboWins ?? null;
    this.trainerJockeyComboPlaces = data.trainerJockeyComboPlaces ?? null;
    this.trainerJockeyComboShows = data.trainerJockeyComboShows ?? null;
    this.trainerJockeyComboRoi = data.trainerJockeyComboRoi ?? null;
    this.daysSinceLastRace = data.daysSinceLastRace ?? null;
    this.lifetimeAllWeatherStarts = data.lifetimeAllWeatherStarts ?? null;
    this.lifetimeAllWeatherWins = data.lifetimeAllWeatherWins ?? null;
    this.lifetimeAllWeatherPlaces = data.lifetimeAllWeatherPlaces ?? null;
    this.lifetimeAllWeatherShows = data.lifetimeAllWeatherShows ?? null;
    this.lifetimeAllWeatherEarnings = data.lifetimeAllWeatherEarnings ?? null;
    this.bestBrisSpeedAllWeatherSurface = data.bestBrisSpeedAllWeatherSurface ?? null;
    this.brisPrimePowerRating = data.brisPrimePowerRating ?? null;
    this.trainerStartsCurrentYear = data.trainerStartsCurrentYear ?? null;
    this.trainerWinsCurrentYear = data.trainerWinsCurrentYear ?? null;
    this.trainerPlacesCurrentYear = data.trainerPlacesCurrentYear ?? null;
    this.trainerShowsCurrentYear = data.trainerShowsCurrentYear ?? null;
    this.trainerRoiCurrentYear = data.trainerRoiCurrentYear ?? null;
    this.trainerStartsPreviousYear = data.trainerStartsPreviousYear ?? null;
    this.trainerWinsPreviousYear = data.trainerWinsPreviousYear ?? null;
    this.trainerPlacesPreviousYear = data.trainerPlacesPreviousYear ?? null;
    this.trainerShowsPreviousYear = data.trainerShowsPreviousYear ?? null;
    this.trainerRoiPreviousYear = data.trainerRoiPreviousYear ?? null;
    this.jockeyStartsCurrentYear = data.jockeyStartsCurrentYear ?? null;
    this.jockeyWinsCurrentYear = data.jockeyWinsCurrentYear ?? null;
    this.jockeyPlacesCurrentYear = data.jockeyPlacesCurrentYear ?? null;
    this.jockeyShowsCurrentYear = data.jockeyShowsCurrentYear ?? null;
    this.jockeyRoiCurrentYear = data.jockeyRoiCurrentYear ?? null;
    this.jockeyStartsPreviousYear = data.jockeyStartsPreviousYear ?? null;
    this.jockeyWinsPreviousYear = data.jockeyWinsPreviousYear ?? null;
    this.jockeyPlacesPreviousYear = data.jockeyPlacesPreviousYear ?? null;
    this.jockeyShowsPreviousYear = data.jockeyShowsPreviousYear ?? null;
    this.jockeyRoiPreviousYear = data.jockeyRoiPreviousYear ?? null;
    this.sireStudFee = data.sireStudFee ?? null;
    this.bestBrisSpeedFastTrack = data.bestBrisSpeedFastTrack ?? null;
    this.bestBrisSpeedTurf = data.bestBrisSpeedTurf ?? null;
    this.bestBrisSpeedOffTrack = data.bestBrisSpeedOffTrack ?? null;
    this.bestBrisSpeedDistance = data.bestBrisSpeedDistance ?? null;
    this.auctionPrice = data.auctionPrice ?? null;
    this.whereWhenSoldAtAuction = data.whereWhenSoldAtAuction ?? '';
    this.brisDirtPedigreeRating = data.brisDirtPedigreeRating ?? '';
    this.brisMudPedigreeRating = data.brisMudPedigreeRating ?? '';
    this.brisTurfPedigreeRating = data.brisTurfPedigreeRating ?? '';
    this.brisDistancePedigreeRating = data.brisDistancePedigreeRating ?? '';
    this.bestBrisSpeedLife = data.bestBrisSpeedLife ?? null;
    this.bestBrisSpeedMostRecentYear = data.bestBrisSpeedMostRecentYear ?? null;
    this.bestBrisSpeed2ndMostRecentYear = data.bestBrisSpeed2ndMostRecentYear ?? null;
    this.bestBrisSpeedTodaysTrack = data.bestBrisSpeedTodaysTrack ?? null;
    this.startsFastDirt = data.startsFastDirt ?? null;
    this.winsFastDirt = data.winsFastDirt ?? null;
    this.placesFastDirt = data.placesFastDirt ?? null;
    this.showsFastDirt = data.showsFastDirt ?? null;
    this.earningsFastDirt = data.earningsFastDirt ?? null;
    this.jockeyDistanceTurfLabel = data.jockeyDistanceTurfLabel ?? '';
    this.jockeyDistanceTurfStarts = data.jockeyDistanceTurfStarts ?? null;
    this.jockeyDistanceTurfWins = data.jockeyDistanceTurfWins ?? null;
    this.jockeyDistanceTurfPlaces = data.jockeyDistanceTurfPlaces ?? null;
    this.jockeyDistanceTurfShows = data.jockeyDistanceTurfShows ?? null;
    this.jockeyDistanceTurfRoi = data.jockeyDistanceTurfRoi ?? null;
    this.jockeyDistanceTurfEarnings = data.jockeyDistanceTurfEarnings ?? null;
    this.trainerJockeyComboStartsMeet = data.trainerJockeyComboStartsMeet ?? null;
    this.trainerJockeyComboWinsMeet = data.trainerJockeyComboWinsMeet ?? null;
    this.trainerJockeyComboPlacesMeet = data.trainerJockeyComboPlacesMeet ?? null;
    this.trainerJockeyComboShowsMeet = data.trainerJockeyComboShowsMeet ?? null;
    this.trainerJockeyComboRoiMeet = data.trainerJockeyComboRoiMeet ?? null;
    this.workouts = data.workouts ?? [];
    this.pastPerformances = data.pastPerformances ?? [];
    this.keyTrainerStats = data.keyTrainerStats ?? [];
  }

  static fromObject(obj: any): Horse {
    return new Horse({
      ...obj,
      workouts: obj.workouts?.map((w: any) => Workout.fromObject(w)) ?? [],
      pastPerformances: obj.pastPerformances?.map((p: any) => PastPerformance.fromObject(p)) ?? [],
      keyTrainerStats: obj.keyTrainerStats?.map((k: any) => KeyTrainerStat.fromObject(k)) ?? []
    });
  }

  toObject(): any {
    return {
      postPosition: this.postPosition,
      entry: this.entry,
      claimingPriceOfHorse: this.claimingPriceOfHorse,
      breedType: this.breedType,
      todaysNasalStripChange: this.todaysNasalStripChange,
      todaysTrainer: this.todaysTrainer,
      trainerStarts: this.trainerStarts,
      trainerWins: this.trainerWins,
      trainerPlaces: this.trainerPlaces,
      trainerShows: this.trainerShows,
      todaysJockey: this.todaysJockey,
      apprenticeWeightAllowance: this.apprenticeWeightAllowance,
      jockeyStarts: this.jockeyStarts,
      jockeyWins: this.jockeyWins,
      jockeyPlaces: this.jockeyPlaces,
      jockeyShows: this.jockeyShows,
      todaysOwner: this.todaysOwner,
      ownersSilks: this.ownersSilks,
      mainTrackOnlyAeIndicator: this.mainTrackOnlyAeIndicator,
      programNumber: this.programNumber,
      morningLineOdds: this.morningLineOdds,
      horseName: this.horseName,
      yearOfBirth: this.yearOfBirth,
      horsesFoalingMonth: this.horsesFoalingMonth,
      sex: this.sex,
      horsesColor: this.horsesColor,
      weight: this.weight,
      sire: this.sire,
      siresSire: this.siresSire,
      dam: this.dam,
      damsSire: this.damsSire,
      breeder: this.breeder,
      stateCountryWhereBred: this.stateCountryWhereBred,
      programPostPosition: this.programPostPosition,
      todaysMedicationNew: this.todaysMedicationNew,
      todaysMedicationOld: this.todaysMedicationOld,
      equipmentChange: this.equipmentChange,
      lifetimeRecordTodaysDistanceStarts: this.lifetimeRecordTodaysDistanceStarts,
      lifetimeRecordTodaysDistanceWins: this.lifetimeRecordTodaysDistanceWins,
      lifetimeRecordTodaysDistancePlaces: this.lifetimeRecordTodaysDistancePlaces,
      lifetimeRecordTodaysDistanceShows: this.lifetimeRecordTodaysDistanceShows,
      lifetimeRecordTodaysDistanceEarnings: this.lifetimeRecordTodaysDistanceEarnings,
      lifetimeRecordTodaysTrackStarts: this.lifetimeRecordTodaysTrackStarts,
      lifetimeRecordTodaysTrackWins: this.lifetimeRecordTodaysTrackWins,
      lifetimeRecordTodaysTrackPlaces: this.lifetimeRecordTodaysTrackPlaces,
      lifetimeRecordTodaysTrackShows: this.lifetimeRecordTodaysTrackShows,
      lifetimeRecordTodaysTrackEarnings: this.lifetimeRecordTodaysTrackEarnings,
      lifetimeRecordTurfStarts: this.lifetimeRecordTurfStarts,
      lifetimeRecordTurfWins: this.lifetimeRecordTurfWins,
      lifetimeRecordTurfPlaces: this.lifetimeRecordTurfPlaces,
      lifetimeRecordTurfShows: this.lifetimeRecordTurfShows,
      lifetimeRecordTurfEarnings: this.lifetimeRecordTurfEarnings,
      lifetimeRecordWetStarts: this.lifetimeRecordWetStarts,
      lifetimeRecordWetWins: this.lifetimeRecordWetWins,
      lifetimeRecordWetPlaces: this.lifetimeRecordWetPlaces,
      lifetimeRecordWetShows: this.lifetimeRecordWetShows,
      lifetimeRecordWetEarnings: this.lifetimeRecordWetEarnings,
      currentYearRecordYear: this.currentYearRecordYear,
      currentYearRecordStarts: this.currentYearRecordStarts,
      currentYearRecordWins: this.currentYearRecordWins,
      currentYearRecordPlaces: this.currentYearRecordPlaces,
      currentYearRecordShows: this.currentYearRecordShows,
      currentYearRecordEarnings: this.currentYearRecordEarnings,
      previousYearRecordYear: this.previousYearRecordYear,
      previousYearRecordStarts: this.previousYearRecordStarts,
      previousYearRecordWins: this.previousYearRecordWins,
      previousYearRecordPlaces: this.previousYearRecordPlaces,
      previousYearRecordShows: this.previousYearRecordShows,
      previousYearRecordEarnings: this.previousYearRecordEarnings,
      lifetimeRecordStarts: this.lifetimeRecordStarts,
      lifetimeRecordWins: this.lifetimeRecordWins,
      lifetimeRecordPlaces: this.lifetimeRecordPlaces,
      lifetimeRecordShows: this.lifetimeRecordShows,
      lifetimeRecordEarnings: this.lifetimeRecordEarnings,
      brisRunStyle: this.brisRunStyle,
      quirinSpeedPoints: this.quirinSpeedPoints,
      trainerJockeyComboStarts: this.trainerJockeyComboStarts,
      trainerJockeyComboWins: this.trainerJockeyComboWins,
      trainerJockeyComboPlaces: this.trainerJockeyComboPlaces,
      trainerJockeyComboShows: this.trainerJockeyComboShows,
      trainerJockeyComboRoi: this.trainerJockeyComboRoi,
      daysSinceLastRace: this.daysSinceLastRace,
      lifetimeAllWeatherStarts: this.lifetimeAllWeatherStarts,
      lifetimeAllWeatherWins: this.lifetimeAllWeatherWins,
      lifetimeAllWeatherPlaces: this.lifetimeAllWeatherPlaces,
      lifetimeAllWeatherShows: this.lifetimeAllWeatherShows,
      lifetimeAllWeatherEarnings: this.lifetimeAllWeatherEarnings,
      bestBrisSpeedAllWeatherSurface: this.bestBrisSpeedAllWeatherSurface,
      brisPrimePowerRating: this.brisPrimePowerRating,
      trainerStartsCurrentYear: this.trainerStartsCurrentYear,
      trainerWinsCurrentYear: this.trainerWinsCurrentYear,
      trainerPlacesCurrentYear: this.trainerPlacesCurrentYear,
      trainerShowsCurrentYear: this.trainerShowsCurrentYear,
      trainerRoiCurrentYear: this.trainerRoiCurrentYear,
      trainerStartsPreviousYear: this.trainerStartsPreviousYear,
      trainerWinsPreviousYear: this.trainerWinsPreviousYear,
      trainerPlacesPreviousYear: this.trainerPlacesPreviousYear,
      trainerShowsPreviousYear: this.trainerShowsPreviousYear,
      trainerRoiPreviousYear: this.trainerRoiPreviousYear,
      jockeyStartsCurrentYear: this.jockeyStartsCurrentYear,
      jockeyWinsCurrentYear: this.jockeyWinsCurrentYear,
      jockeyPlacesCurrentYear: this.jockeyPlacesCurrentYear,
      jockeyShowsCurrentYear: this.jockeyShowsCurrentYear,
      jockeyRoiCurrentYear: this.jockeyRoiCurrentYear,
      jockeyStartsPreviousYear: this.jockeyStartsPreviousYear,
      jockeyWinsPreviousYear: this.jockeyWinsPreviousYear,
      jockeyPlacesPreviousYear: this.jockeyPlacesPreviousYear,
      jockeyShowsPreviousYear: this.jockeyShowsPreviousYear,
      jockeyRoiPreviousYear: this.jockeyRoiPreviousYear,
      sireStudFee: this.sireStudFee,
      bestBrisSpeedFastTrack: this.bestBrisSpeedFastTrack,
      bestBrisSpeedTurf: this.bestBrisSpeedTurf,
      bestBrisSpeedOffTrack: this.bestBrisSpeedOffTrack,
      bestBrisSpeedDistance: this.bestBrisSpeedDistance,
      auctionPrice: this.auctionPrice,
      whereWhenSoldAtAuction: this.whereWhenSoldAtAuction,
      brisDirtPedigreeRating: this.brisDirtPedigreeRating,
      brisMudPedigreeRating: this.brisMudPedigreeRating,
      brisTurfPedigreeRating: this.brisTurfPedigreeRating,
      brisDistancePedigreeRating: this.brisDistancePedigreeRating,
      bestBrisSpeedLife: this.bestBrisSpeedLife,
      bestBrisSpeedMostRecentYear: this.bestBrisSpeedMostRecentYear,
      bestBrisSpeed2ndMostRecentYear: this.bestBrisSpeed2ndMostRecentYear,
      bestBrisSpeedTodaysTrack: this.bestBrisSpeedTodaysTrack,
      startsFastDirt: this.startsFastDirt,
      winsFastDirt: this.winsFastDirt,
      placesFastDirt: this.placesFastDirt,
      showsFastDirt: this.showsFastDirt,
      earningsFastDirt: this.earningsFastDirt,
      jockeyDistanceTurfLabel: this.jockeyDistanceTurfLabel,
      jockeyDistanceTurfStarts: this.jockeyDistanceTurfStarts,
      jockeyDistanceTurfWins: this.jockeyDistanceTurfWins,
      jockeyDistanceTurfPlaces: this.jockeyDistanceTurfPlaces,
      jockeyDistanceTurfShows: this.jockeyDistanceTurfShows,
      jockeyDistanceTurfRoi: this.jockeyDistanceTurfRoi,
      jockeyDistanceTurfEarnings: this.jockeyDistanceTurfEarnings,
      trainerJockeyComboStartsMeet: this.trainerJockeyComboStartsMeet,
      trainerJockeyComboWinsMeet: this.trainerJockeyComboWinsMeet,
      trainerJockeyComboPlacesMeet: this.trainerJockeyComboPlacesMeet,
      trainerJockeyComboShowsMeet: this.trainerJockeyComboShowsMeet,
      trainerJockeyComboRoiMeet: this.trainerJockeyComboRoiMeet,
      workouts: this.workouts.map(w => w.toObject()),
      pastPerformances: this.pastPerformances.map(p => p.toObject()),
      keyTrainerStats: this.keyTrainerStats.map(k => k.toObject())
    };
  }
}

export class Workout {
  date: string;
  time: number | null;
  track: string;
  distance: number | null;
  condition: string;
  description: string;
  mainInnerTrackIndicator: string;
  workoutsThatDayDistance: number | null;
  rank: number | null;

  constructor(
    date: string = '',
    time: number | null = null,
    track: string = '',
    distance: number | null = null,
    condition: string = '',
    description: string = '',
    mainInnerTrackIndicator: string = '',
    workoutsThatDayDistance: number | null = null,
    rank: number | null = null
  ) {
    this.date = date;
    this.time = time;
    this.track = track;
    this.distance = distance;
    this.condition = condition;
    this.description = description;
    this.mainInnerTrackIndicator = mainInnerTrackIndicator;
    this.workoutsThatDayDistance = workoutsThatDayDistance;
    this.rank = rank;
  }

  static fromObject(obj: any): Workout {
    return new Workout(
      obj.date ?? '',
      obj.time ?? null,
      obj.track ?? '',
      obj.distance ?? null,
      obj.condition ?? '',
      obj.description ?? '',
      obj.mainInnerTrackIndicator ?? '',
      obj.workoutsThatDayDistance ?? null,
      obj.rank ?? null
    );
  }

  toObject(): any {
    return {
      date: this.date,
      time: this.time,
      track: this.track,
      distance: this.distance,
      condition: this.condition,
      description: this.description,
      mainInnerTrackIndicator: this.mainInnerTrackIndicator,
      workoutsThatDayDistance: this.workoutsThatDayDistance,
      rank: this.rank
    };
  }
}

export class KeyTrainerStat {
  category: string;
  starts: number | null;
  winPct: number | null;
  inTheMoneyPct: number | null;
  roi: number | null;

  constructor(data: any = {}) {
    this.category = data.category ?? '';
    this.starts = data.starts ?? null;
    this.winPct = data.winPct ?? null;
    this.inTheMoneyPct = data.inTheMoneyPct ?? null;
    this.roi = data.roi ?? null;
  }

  static fromObject(obj: any): KeyTrainerStat {
    return new KeyTrainerStat({
      category: obj.category,
      starts: obj.starts,
      winPct: obj.winPct,
      inTheMoneyPct: obj.inTheMoneyPct,
      roi: obj.roi
    });
  }

  toObject(): any {
    return {
      category: this.category,
      starts: this.starts,
      winPct: this.winPct,
      inTheMoneyPct: this.inTheMoneyPct,
      roi: this.roi
    };
  }
}

export class PastPerformance {
  raceDate: string;
  daysSinceLastRace: number | null;
  trackCode: string;
  brisTrackCode: string;
  raceNumber: number | null;
  trackCondition: string;
  distance: number | null;
  surface: string;
  specialChuteIndicator: string;
  entrants: number | null;
  postPosition: number | null;
  equipment: string;
  racename: string;
  medication: number | null;
  tripComment: string;
  winnersName: string;
  placeName: string;
  showName: string;
  winnersWeight: number | null;
  placeWeight: number | null;
  showWeight: number | null;
  winnersMargin: number | null;
  placeMargin: number | null;
  showMargin: number | null;
  alternateCommentLine: string;
  weight: number | null;
  odds: number | null;
  entry: string;
  raceClassication: string;
  claimingPrice: number | null;
  purse: number | null;
  startCallPosition: string;
  firstCallPosition: string;
  secondCallPosition: string;
  gateCallPosition: string;
  stretchCallPosition: string;
  finishPosition: string;
  moneyPosition: string;
  startCallBetweenLengthsLeader: number | null;
  startCallBetweenLengths: number | null;
  firstCallBetweenLengthsLeader: number | null;
  firstCallBetweenLengths: number | null;
  secondCallBetweenLengthsLeader: number | null;
  secondCallBetweenLengths: number | null;
  brisRaceShape1stCall: number | null;
  stretchCallBetweenLengthsLeader: number | null;
  stretchCallBetweenLengths: number | null;
  finishBetweenLengthsLeader: number | null;
  finishBetweenLengths: number | null;
  brisRaceShape2ndCall: number | null;
  bris2fPace: number | null;
  bris4fPace: number | null;
  bris6fPace: number | null;
  bris8fPace: number | null;
  bris10fPace: number | null;
  brisLatePace: number | null;
  brisSpeedRating: number | null;
  speedRating: number | null;
  trackVariant: number | null;
  twoFFraction: number | null;
  threeFFraction: number | null;
  fourFFraction: number | null;
  fiveFFraction: number | null;
  sixFFraction: number | null;
  sevenFFraction: number | null;
  eightFFraction: number | null;
  tenFFraction: number | null;
  twelveFFraction: number | null;
  fourteenFFraction: number | null;
  sixteenFFraction: number | null;
  fraction1: number | null;
  fraction2: number | null;
  fraction3: number | null;
  finalTime: number | null;
  claimedCode: string;
  trainer: string;
  jockey: string;
  apprenticeWeightAllowance: number | null;
  raceType: string;
  ageSexRestrictions: string;
  statebredFlag: string;
  restrictedQualifierFlag: string;
  favoriteIndicator: string;
  frontBandagesIndicator: string;
  brisSpeedParForRace: number | null;
  barShoes: string;
  companyLineCodes: string;
  lowClaimingPriceOfRace: number | null;
  highClaimingPriceOfRace: number | null;
  codeForPriorRaces: string;
  claimedAndTrainerSwitches1: string;
  claimedAndTrainerSwitches2: string;
  claimedAndTrainerSwitches3: string;
  claimedAndTrainerSwitches4: string;
  claimedAndTrainerSwitches5: string;
  claimedAndTrainerSwitches6: string;
  extendedStartComment: string;
  sealedTrackIndicator: string;
  previousAllWeatherSurfaceIndicator: string;
  equibaseAbbreviatedRaceCondition: string;

  constructor(data: any = {}) {
    this.raceDate = data.raceDate ?? '';
    this.daysSinceLastRace = data.daysSinceLastRace ?? null;
    this.trackCode = data.trackCode ?? '';
    this.brisTrackCode = data.brisTrackCode ?? '';
    this.raceNumber = data.raceNumber ?? null;
    this.trackCondition = data.trackCondition ?? '';
    this.distance = data.distance ?? null;
    this.surface = data.surface ?? '';
    this.specialChuteIndicator = data.specialChuteIndicator ?? '';
    this.entrants = data.entrants ?? null;
    this.postPosition = data.postPosition ?? null;
    this.equipment = data.equipment ?? '';
    this.racename = data.racename ?? '';
    this.medication = data.medication ?? null;
    this.tripComment = data.tripComment ?? '';
    this.winnersName = data.winnersName ?? '';
    this.placeName = data.placeName ?? '';
    this.showName = data.showName ?? '';
    this.winnersWeight = data.winnersWeight ?? null;
    this.placeWeight = data.placeWeight ?? null;
    this.showWeight = data.showWeight ?? null;
    this.winnersMargin = data.winnersMargin ?? null;
    this.placeMargin = data.placeMargin ?? null;
    this.showMargin = data.showMargin ?? null;
    this.alternateCommentLine = data.alternateCommentLine ?? '';
    this.weight = data.weight ?? null;
    this.odds = data.odds ?? null;
    this.entry = data.entry ?? '';
    this.raceClassication = data.raceClassication ?? '';
    this.claimingPrice = data.claimingPrice ?? null;
    this.purse = data.purse ?? null;
    this.startCallPosition = data.startCallPosition ?? '';
    this.firstCallPosition = data.firstCallPosition ?? '';
    this.secondCallPosition = data.secondCallPosition ?? '';
    this.gateCallPosition = data.gateCallPosition ?? '';
    this.stretchCallPosition = data.stretchCallPosition ?? '';
    this.finishPosition = data.finishPosition ?? '';
    this.moneyPosition = data.moneyPosition ?? '';
    this.startCallBetweenLengthsLeader = data.startCallBetweenLengthsLeader ?? null;
    this.startCallBetweenLengths = data.startCallBetweenLengths ?? null;
    this.firstCallBetweenLengthsLeader = data.firstCallBetweenLengthsLeader ?? null;
    this.firstCallBetweenLengths = data.firstCallBetweenLengths ?? null;
    this.secondCallBetweenLengthsLeader = data.secondCallBetweenLengthsLeader ?? null;
    this.secondCallBetweenLengths = data.secondCallBetweenLengths ?? null;
    this.brisRaceShape1stCall = data.brisRaceShape1stCall ?? null;
    this.stretchCallBetweenLengthsLeader = data.stretchCallBetweenLengthsLeader ?? null;
    this.stretchCallBetweenLengths = data.stretchCallBetweenLengths ?? null;
    this.finishBetweenLengthsLeader = data.finishBetweenLengthsLeader ?? null;
    this.finishBetweenLengths = data.finishBetweenLengths ?? null;
    this.brisRaceShape2ndCall = data.brisRaceShape2ndCall ?? null;
    this.bris2fPace = data.bris2fPace ?? null;
    this.bris4fPace = data.bris4fPace ?? null;
    this.bris6fPace = data.bris6fPace ?? null;
    this.bris8fPace = data.bris8fPace ?? null;
    this.bris10fPace = data.bris10fPace ?? null;
    this.brisLatePace = data.brisLatePace ?? null;
    this.brisSpeedRating = data.brisSpeedRating ?? null;
    this.speedRating = data.speedRating ?? null;
    this.trackVariant = data.trackVariant ?? null;
    this.twoFFraction = data.twoFFraction ?? null;
    this.threeFFraction = data.threeFFraction ?? null;
    this.fourFFraction = data.fourFFraction ?? null;
    this.fiveFFraction = data.fiveFFraction ?? null;
    this.sixFFraction = data.sixFFraction ?? null;
    this.sevenFFraction = data.sevenFFraction ?? null;
    this.eightFFraction = data.eightFFraction ?? null;
    this.tenFFraction = data.tenFFraction ?? null;
    this.twelveFFraction = data.twelveFFraction ?? null;
    this.fourteenFFraction = data.fourteenFFraction ?? null;
    this.sixteenFFraction = data.sixteenFFraction ?? null;
    this.fraction1 = data.fraction1 ?? null;
    this.fraction2 = data.fraction2 ?? null;
    this.fraction3 = data.fraction3 ?? null;
    this.finalTime = data.finalTime ?? null;
    this.claimedCode = data.claimedCode ?? '';
    this.trainer = data.trainer ?? '';
    this.jockey = data.jockey ?? '';
    this.apprenticeWeightAllowance = data.apprenticeWeightAllowance ?? null;
    this.raceType = data.raceType ?? '';
    this.ageSexRestrictions = data.ageSexRestrictions ?? '';
    this.statebredFlag = data.statebredFlag ?? '';
    this.restrictedQualifierFlag = data.restrictedQualifierFlag ?? '';
    this.favoriteIndicator = data.favoriteIndicator ?? '';
    this.frontBandagesIndicator = data.frontBandagesIndicator ?? '';
    this.brisSpeedParForRace = data.brisSpeedParForRace ?? null;
    this.barShoes = data.barShoes ?? '';
    this.companyLineCodes = data.companyLineCodes ?? '';
    this.lowClaimingPriceOfRace = data.lowClaimingPriceOfRace ?? null;
    this.highClaimingPriceOfRace = data.highClaimingPriceOfRace ?? null;
    this.codeForPriorRaces = data.codeForPriorRaces ?? '';
    this.claimedAndTrainerSwitches1 = data.claimedAndTrainerSwitches1 ?? '';
    this.claimedAndTrainerSwitches2 = data.claimedAndTrainerSwitches2 ?? '';
    this.claimedAndTrainerSwitches3 = data.claimedAndTrainerSwitches3 ?? '';
    this.claimedAndTrainerSwitches4 = data.claimedAndTrainerSwitches4 ?? '';
    this.claimedAndTrainerSwitches5 = data.claimedAndTrainerSwitches5 ?? '';
    this.claimedAndTrainerSwitches6 = data.claimedAndTrainerSwitches6 ?? '';
    this.extendedStartComment = data.extendedStartComment ?? '';
    this.sealedTrackIndicator = data.sealedTrackIndicator ?? '';
    this.previousAllWeatherSurfaceIndicator = data.previousAllWeatherSurfaceIndicator ?? '';
    this.equibaseAbbreviatedRaceCondition = data.equibaseAbbreviatedRaceCondition ?? '';
  }

  static fromObject(obj: any): PastPerformance {
    return new PastPerformance(obj);
  }

  toObject(): any {
    return {
      raceDate: this.raceDate,
      daysSinceLastRace: this.daysSinceLastRace,
      trackCode: this.trackCode,
      brisTrackCode: this.brisTrackCode,
      raceNumber: this.raceNumber,
      trackCondition: this.trackCondition,
      distance: this.distance,
      surface: this.surface,
      specialChuteIndicator: this.specialChuteIndicator,
      entrants: this.entrants,
      postPosition: this.postPosition,
      equipment: this.equipment,
      racename: this.racename,
      medication: this.medication,
      tripComment: this.tripComment,
      winnersName: this.winnersName,
      placeName: this.placeName,
      showName: this.showName,
      winnersWeight: this.winnersWeight,
      placeWeight: this.placeWeight,
      showWeight: this.showWeight,
      winnersMargin: this.winnersMargin,
      placeMargin: this.placeMargin,
      showMargin: this.showMargin,
      alternateCommentLine: this.alternateCommentLine,
      weight: this.weight,
      odds: this.odds,
      entry: this.entry,
      raceClassication: this.raceClassication,
      claimingPrice: this.claimingPrice,
      purse: this.purse,
      startCallPosition: this.startCallPosition,
      firstCallPosition: this.firstCallPosition,
      secondCallPosition: this.secondCallPosition,
      gateCallPosition: this.gateCallPosition,
      stretchCallPosition: this.stretchCallPosition,
      finishPosition: this.finishPosition,
      moneyPosition: this.moneyPosition,
      startCallBetweenLengthsLeader: this.startCallBetweenLengthsLeader,
      startCallBetweenLengths: this.startCallBetweenLengths,
      firstCallBetweenLengthsLeader: this.firstCallBetweenLengthsLeader,
      firstCallBetweenLengths: this.firstCallBetweenLengths,
      secondCallBetweenLengthsLeader: this.secondCallBetweenLengthsLeader,
      secondCallBetweenLengths: this.secondCallBetweenLengths,
      brisRaceShape1stCall: this.brisRaceShape1stCall,
      stretchCallBetweenLengthsLeader: this.stretchCallBetweenLengthsLeader,
      stretchCallBetweenLengths: this.stretchCallBetweenLengths,
      finishBetweenLengthsLeader: this.finishBetweenLengthsLeader,
      finishBetweenLengths: this.finishBetweenLengths,
      brisRaceShape2ndCall: this.brisRaceShape2ndCall,
      bris2fPace: this.bris2fPace,
      bris4fPace: this.bris4fPace,
      bris6fPace: this.bris6fPace,
      bris8fPace: this.bris8fPace,
      bris10fPace: this.bris10fPace,
      brisLatePace: this.brisLatePace,
      brisSpeedRating: this.brisSpeedRating,
      speedRating: this.speedRating,
      trackVariant: this.trackVariant,
      twoFFraction: this.twoFFraction,
      threeFFraction: this.threeFFraction,
      fourFFraction: this.fourFFraction,
      fiveFFraction: this.fiveFFraction,
      sixFFraction: this.sixFFraction,
      sevenFFraction: this.sevenFFraction,
      eightFFraction: this.eightFFraction,
      tenFFraction: this.tenFFraction,
      twelveFFraction: this.twelveFFraction,
      fourteenFFraction: this.fourteenFFraction,
      sixteenFFraction: this.sixteenFFraction,
      fraction1: this.fraction1,
      fraction2: this.fraction2,
      fraction3: this.fraction3,
      finalTime: this.finalTime,
      claimedCode: this.claimedCode,
      trainer: this.trainer,
      jockey: this.jockey,
      apprenticeWeightAllowance: this.apprenticeWeightAllowance,
      raceType: this.raceType,
      ageSexRestrictions: this.ageSexRestrictions,
      statebredFlag: this.statebredFlag,
      restrictedQualifierFlag: this.restrictedQualifierFlag,
      favoriteIndicator: this.favoriteIndicator,
      frontBandagesIndicator: this.frontBandagesIndicator,
      brisSpeedParForRace: this.brisSpeedParForRace,
      barShoes: this.barShoes,
      companyLineCodes: this.companyLineCodes,
      lowClaimingPriceOfRace: this.lowClaimingPriceOfRace,
      highClaimingPriceOfRace: this.highClaimingPriceOfRace,
      codeForPriorRaces: this.codeForPriorRaces,
      claimedAndTrainerSwitches1: this.claimedAndTrainerSwitches1,
      claimedAndTrainerSwitches2: this.claimedAndTrainerSwitches2,
      claimedAndTrainerSwitches3: this.claimedAndTrainerSwitches3,
      claimedAndTrainerSwitches4: this.claimedAndTrainerSwitches4,
      claimedAndTrainerSwitches5: this.claimedAndTrainerSwitches5,
      claimedAndTrainerSwitches6: this.claimedAndTrainerSwitches6,
      extendedStartComment: this.extendedStartComment,
      sealedTrackIndicator: this.sealedTrackIndicator,
      previousAllWeatherSurfaceIndicator: this.previousAllWeatherSurfaceIndicator,
      equibaseAbbreviatedRaceCondition: this.equibaseAbbreviatedRaceCondition
    };
  }
}
