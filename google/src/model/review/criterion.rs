#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "strict", derive(Copy))]
#[serde(deny_unknown_fields)]
pub enum CriterionType {
    #[serde(rename = "vaf_2nd_screen_experience")]
    SecondScreenExperience,
    #[serde(rename = "vaf_4k_content")]
    FourKContent,
    #[serde(rename = "vaf_4k_movies_shows")]
    FourKMoviesShows,
    #[serde(rename = "vaf_actor_bios")]
    ActorBios,
    #[serde(rename = "vaf_age_targeting_6_8")]
    AgeTargeting68,
    #[serde(rename = "vaf_android_tv")]
    AndroidTv,
    #[serde(rename = "vaf_apps_attribute_anime_v1")]
    AppsAttributeAnimeV1,
    #[serde(rename = "vaf_apps_attribute_anime_v2")]
    AppsAttributeAnimeV2,
    #[serde(rename = "vaf_apps_attribute_download_for_offline_viewing_v1")]
    AppsAttributeDownloadForOfflineViewingV1,
    #[serde(rename = "vaf_apps_attribute_download_for_offline_viewing_v2")]
    AppsAttributeDownloadForOfflineViewingV2,
    #[serde(rename = "vaf_apps_attribute_face_swap_v1")]
    AppsAttributeFaceSwapV1,
    #[serde(rename = "vaf_apps_attribute_face_swap_v2")]
    AppsAttributeFaceSwapV2,
    #[serde(rename = "vaf_apps_attribute_live_v1")]
    AppsAttributeLiveV1,
    #[serde(rename = "vaf_apps_attribute_live_v2")]
    AppsAttributeLiveV2,
    #[serde(rename = "vaf_apps_attribute_sports_features_out_of_network_v1")]
    AppsAttributeSportsFeaturesOutOfNetworkV1,
    #[serde(rename = "vaf_apps_attribute_sports_features_out_of_network_v2")]
    AppsAttributeSportsFeaturesOutOfNetworkV2,
    #[serde(rename = "vaf_apps_category_food_and_drink_v1")]
    AppsCategoryFoodAndDrinkV1,
    #[serde(rename = "vaf_apps_category_food_and_drink_v2")]
    AppsCategoryFoodAndDrinkV2,
    #[serde(rename = "vaf_apps_category_restaurant_v1")]
    AppsCategoryRestaurantV1,
    #[serde(rename = "vaf_apps_category_restaurant_v2")]
    AppsCategoryRestaurantV2,
    #[serde(rename = "vaf_auto_play")]
    AutoPlay,
    #[serde(rename = "vaf_available_social_networks")]
    AvailableSocialNetworks,
    #[serde(rename = "vaf_backing_up_photos")]
    BackingUpPhotos,
    #[serde(rename = "vaf_blur_feature")]
    BlurFeature,
    #[serde(rename = "vaf_buying_movies_shows")]
    BuyingMoviesShows,
    #[serde(rename = "vaf_buying_tickets")]
    BuyingTickets,
    #[serde(rename = "vaf_camera")]
    Camera,
    #[serde(rename = "vaf_cancellation_policy")]
    CancellationPolicy,
    #[serde(rename = "vaf_changing_reservations")]
    ChangingReservations,
    #[serde(rename = "vaf_closed_captions")]
    ClosedCaptions,
    #[serde(rename = "vaf_creating_albums")]
    CreatingAlbums,
    #[serde(rename = "vaf_creating_gifs")]
    CreatingGifs,
    #[serde(rename = "vaf_cropping_photos")]
    CroppingPhotos,
    #[serde(rename = "vaf_delete_messages")]
    DeleteMessages,
    #[serde(rename = "vaf_dictionary")]
    Dictionary,
    #[serde(rename = "vaf_editing_photos")]
    EditingPhotos,
    #[serde(rename = "vaf_editing_text")]
    EditingText,
    #[serde(rename = "vaf_emojis")]
    Emojis,
    #[serde(rename = "vaf_equalizer")]
    Equalizer,
    #[serde(rename = "vaf_filmographies")]
    Filmographies,
    #[serde(rename = "vaf_filters")]
    Filters,
    #[serde(rename = "vaf_free_content")]
    FreeContent,
    #[serde(rename = "vaf_free_storage")]
    FreeStorage,
    #[serde(rename = "vaf_free_trial")]
    FreeTrial,
    #[serde(rename = "vaf_games_addictive")]
    GamesAddictive,
    #[serde(rename = "vaf_games_beautiful")]
    GamesBeautiful,
    #[serde(rename = "vaf_games_challenging")]
    GamesChallenging,
    #[serde(rename = "vaf_games_classic")]
    GamesClassic,
    #[serde(rename = "vaf_games_complex")]
    GamesComplex,
    #[serde(rename = "vaf_games_connectivity_required_before_first_use")]
    GamesConnectivityRequiredBeforeFirstUse,
    #[serde(rename = "vaf_games_customizable")]
    GamesCustomizable,
    #[serde(rename = "vaf_games_cute")]
    GamesCute,
    #[serde(rename = "vaf_games_dimensional_plane_3d_v1")]
    GamesDimensionalPlane3dV1,
    #[serde(rename = "vaf_games_dimensional_plane_3d_v2")]
    GamesDimensionalPlane3dV2,
    #[serde(rename = "vaf_games_educational")]
    GamesEducational,
    #[serde(rename = "vaf_games_entertaining")]
    GamesEntertaining,
    #[serde(rename = "vaf_games_exciting")]
    GamesExciting,
    #[serde(rename = "vaf_games_feature_leaderboard")]
    GamesFeatureLeaderboard,
    #[serde(rename = "vaf_games_fun")]
    GamesFun,
    #[serde(rename = "vaf_games_funny")]
    GamesFunny,
    #[serde(rename = "vaf_games_genre_4x_explore_expand_exploit_and_exterminate")]
    GamesGenre4xExploreExpandExploitAndExterminate,
    #[serde(rename = "vaf_games_genre_abstract_strategy")]
    GamesGenreAbstractStrategy,
    #[serde(rename = "vaf_games_genre_action")]
    GamesGenreAction,
    #[serde(rename = "vaf_games_genre_action_adventure")]
    GamesGenreActionAdventure,
    #[serde(rename = "vaf_games_genre_action_rpg")]
    GamesGenreActionRpg,
    #[serde(rename = "vaf_games_genre_adventure")]
    GamesGenreAdventure,
    #[serde(rename = "vaf_games_genre_arcade")]
    GamesGenreArcade,
    #[serde(rename = "vaf_games_genre_artillery")]
    GamesGenreArtillery,
    #[serde(rename = "vaf_games_genre_ball")]
    GamesGenreBall,
    #[serde(rename = "vaf_games_genre_base_building")]
    GamesGenreBaseBuilding,
    #[serde(rename = "vaf_games_genre_board")]
    GamesGenreBoard,
    #[serde(rename = "vaf_games_genre_brain_teaser")]
    GamesGenreBrainTeaser,
    #[serde(rename = "vaf_games_genre_brain_training")]
    GamesGenreBrainTraining,
    #[serde(rename = "vaf_games_genre_bubble_shooter")]
    GamesGenreBubbleShooter,
    #[serde(rename = "vaf_games_genre_card")]
    GamesGenreCard,
    #[serde(rename = "vaf_games_genre_casual")]
    GamesGenreCasual,
    #[serde(rename = "vaf_games_genre_city_building")]
    GamesGenreCityBuilding,
    #[serde(rename = "vaf_games_genre_claw")]
    GamesGenreClaw,
    #[serde(rename = "vaf_games_genre_construction_and_management_simulation")]
    GamesGenreConstructionAndManagementSimulation,
    #[serde(rename = "vaf_games_genre_crosswords")]
    GamesGenreCrosswords,
    #[serde(rename = "vaf_games_genre_dating_simulation")]
    GamesGenreDatingSimulation,
    #[serde(rename = "vaf_games_genre_dice")]
    GamesGenreDice,
    #[serde(rename = "vaf_games_genre_dress_up_and_fashion")]
    GamesGenreDressUpAndFashion,
    #[serde(rename = "vaf_games_genre_dungeon_crawlers")]
    GamesGenreDungeonCrawlers,
    #[serde(rename = "vaf_games_genre_empire_building")]
    GamesGenreEmpireBuilding,
    #[serde(rename = "vaf_games_genre_endless_runners")]
    GamesGenreEndlessRunners,
    #[serde(rename = "vaf_games_genre_exploration")]
    GamesGenreExploration,
    #[serde(rename = "vaf_games_genre_fantasy_rpg")]
    GamesGenreFantasyRpg,
    #[serde(rename = "vaf_games_genre_fighting")]
    GamesGenreFighting,
    #[serde(rename = "vaf_games_genre_first_person_action_adventure")]
    GamesGenreFirstActionAdventure,
    #[serde(rename = "vaf_games_genre_first_person_shooter")]
    GamesGenreFirstPersonShooter,
    #[serde(rename = "vaf_games_genre_government_simulation")]
    GamesGenreGovernmentSimulation,
    #[serde(rename = "vaf_games_genre_graphic_adventure")]
    GamesGenreGraphicAdventure,
    #[serde(rename = "vaf_games_genre_guessing")]
    GamesGenreGuessing,
    #[serde(rename = "vaf_games_genre_hack_and_slash")]
    GamesGenreHackAndSlash,
    #[serde(rename = "vaf_games_genre_hero_shooter")]
    GamesGenreHeroShooter,
    #[serde(rename = "vaf_games_genre_horror")]
    GamesGenreHorror,
    #[serde(rename = "vaf_games_genre_incremental")]
    GamesGenreIncremental,
    #[serde(rename = "vaf_games_genre_interactive_fiction")]
    GamesGenreInteractiveFiction,
    #[serde(rename = "vaf_games_genre_isometric_platform")]
    GamesGenreIsometricPlatform,
    #[serde(rename = "vaf_games_genre_japanese_rpg")]
    GamesGenreJapaneseRpg,
    #[serde(rename = "vaf_games_genre_jigsaw")]
    GamesGenreJigsaw,
    #[serde(rename = "vaf_games_genre_life_simulation")]
    GamesGenreLifeSimulation,
    #[serde(rename = "vaf_games_genre_logic")]
    GamesGenreLogic,
    #[serde(rename = "vaf_games_genre_massively_multiplayer_online")]
    GamesGenreMassivelyMultiplayerOnline,
    #[serde(rename = "vaf_games_genre_massively_multiplayer_online_first_person_shooter")]
    GamesGenreMassivelyMultiplayerOnlineFirstPersonShooter,
    #[serde(rename = "vaf_games_genre_massively_multiplayer_online_role_playing")]
    GamesGenreMassivelyMultiplayerOnlineRolePlaying,
    #[serde(rename = "vaf_games_genre_maze")]
    GamesGenreMaze,
    #[serde(rename = "vaf_games_genre_medical_simulation")]
    GamesGenreMedicalSimulation,
    #[serde(rename = "vaf_games_genre_memory")]
    GamesGenreMemory,
    #[serde(rename = "vaf_games_genre_military_strategy")]
    GamesGenreMilitaryStrategy,
    #[serde(rename = "vaf_games_genre_minigames")]
    GamesGenreMinigames,
    #[serde(rename = "vaf_games_genre_multiplayer_online_battle_arena")]
    GamesGenreMultiplayerOnlineBattleArena,
    #[serde(rename = "vaf_games_genre_mystery_adventure")]
    GamesGenreMysteryAdventure,
    #[serde(rename = "vaf_games_genre_platform_adventure")]
    GamesGenrePlatformAdventure,
    #[serde(rename = "vaf_games_genre_platformers")]
    GamesGenrePlatformers,
    #[serde(rename = "vaf_games_genre_puzzle")]
    GamesGenrePuzzle,
    #[serde(rename = "vaf_games_genre_puzzle_v1")]
    GamesGenrePuzzleV1,
    #[serde(rename = "vaf_games_genre_puzzle_v2")]
    GamesGenrePuzzleV2,
    #[serde(rename = "vaf_games_genre_racing")]
    GamesGenreRacing,
    #[serde(rename = "vaf_games_genre_real_time_strategy")]
    GamesGenreRealTimeStrategy,
    #[serde(rename = "vaf_games_genre_real_time_tactics")]
    GamesGenreRealTimeTactics,
    #[serde(rename = "vaf_games_genre_restaurant")]
    GamesGenreRestaurant,
    #[serde(rename = "vaf_games_genre_reveal_the_picture")]
    GamesGenreRevealThePicture,
    #[serde(rename = "vaf_games_genre_rhythm")]
    GamesGenreRhythm,
    #[serde(rename = "vaf_games_genre_roguelike")]
    GamesGenreRoguelike,
    #[serde(rename = "vaf_games_genre_role_playing")]
    GamesGenreRolePlaying,
    #[serde(rename = "vaf_games_genre_room_escape")]
    GamesGenreRoomEscape,
    #[serde(rename = "vaf_games_genre_rummy")]
    GamesGenreRummy,
    #[serde(rename = "vaf_games_genre_sandbox_and_open_world_rpg")]
    GamesGenreSandboxAndOpenWorldRpg,
    #[serde(rename = "vaf_games_genre_science_fiction")]
    GamesGenreScienceFiction,
    #[serde(rename = "vaf_games_genre_shoot_em_up")]
    GamesGenreShootEmUp,
    #[serde(rename = "vaf_games_genre_shooting")]
    GamesGenreShooting,
    #[serde(rename = "vaf_games_genre_shooting_gallery")]
    GamesGenreShootingGallery,
    #[serde(rename = "vaf_games_genre_sim_racing")]
    GamesGenreSimRacing,
    #[serde(rename = "vaf_games_genre_simulation")]
    GamesGenreSimulation,
    #[serde(rename = "vaf_games_genre_sliding_puzzle")]
    GamesGenreSlidingPuzzle,
    #[serde(rename = "vaf_games_genre_slot_machine")]
    GamesGenreSlotMachine,
    #[serde(rename = "vaf_games_genre_social_simulation")]
    GamesGenreSocialSimulation,
    #[serde(rename = "vaf_games_genre_sokoban")]
    GamesGenreSokoban,
    #[serde(rename = "vaf_games_genre_stealth")]
    GamesGenreStealth,
    #[serde(rename = "vaf_games_genre_strategy")]
    GamesGenreStrategy,
    #[serde(rename = "vaf_games_genre_strategy_rpg")]
    GamesGenreStrategyRpg,
    #[serde(rename = "vaf_games_genre_stunt_driving")]
    GamesGenreStuntDriving,
    #[serde(rename = "vaf_games_genre_survival")]
    GamesGenreSurvival,
    #[serde(rename = "vaf_games_genre_survival_horror")]
    GamesGenreSurvivalHorror,
    #[serde(rename = "vaf_games_genre_table")]
    GamesGenreTable,
    #[serde(rename = "vaf_games_genre_tactical_rpg")]
    GamesGenreTacticalRpg,
    #[serde(rename = "vaf_games_genre_tactical_shooter")]
    GamesGenreTacticalShooter,
    #[serde(rename = "vaf_games_genre_third_person_action_adventure")]
    GamesGenreThirdPersonActionAdventure,
    #[serde(rename = "vaf_games_genre_third_person_shooter")]
    GamesGenreThirdPersonShooter,
    #[serde(rename = "vaf_games_genre_tile_matching")]
    GamesGenreTileMatching,
    #[serde(rename = "vaf_games_genre_trading_card")]
    GamesGenreTradingCard,
    #[serde(rename = "vaf_games_genre_trivia")]
    GamesGenreTrivia,
    #[serde(rename = "vaf_games_genre_turn_based_rpg_v1")]
    GamesGenreTurnBasedRpgV1,
    #[serde(rename = "vaf_games_genre_turn_based_rpg_v2")]
    GamesGenreTurnBasedRpgV2,
    #[serde(rename = "vaf_games_genre_turn_based_strategy")]
    GamesGenreTurnBasedStrategy,
    #[serde(rename = "vaf_games_genre_turn_based_tactics")]
    GamesGenreTurnBasedTactics,
    #[serde(rename = "vaf_games_genre_tycoon")]
    GamesGenreTycoon,
    #[serde(rename = "vaf_games_genre_wargames")]
    GamesGenreWargames,
    #[serde(rename = "vaf_games_graphic_style_anime_v1")]
    GamesGraphicStyleAnimeV1,
    #[serde(rename = "vaf_games_graphic_style_anime_v2")]
    GamesGraphicStyleAnimeV2,
    #[serde(rename = "vaf_games_graphic_style_cartoon_v1")]
    GamesGraphicStyleCartoonV1,
    #[serde(rename = "vaf_games_graphic_style_cartoon_v2")]
    GamesGraphicStyleCartoonV2,
    #[serde(rename = "vaf_games_graphic_style_pixelated_v1")]
    GamesGraphicStylePixelatedV1,
    #[serde(rename = "vaf_games_graphic_style_pixelated_v2")]
    GamesGraphicStylePixelatedV2,
    #[serde(rename = "vaf_games_graphic_style_stylized_v1")]
    GamesGraphicStyleStylizedV1,
    #[serde(rename = "vaf_games_graphic_style_stylized_v2")]
    GamesGraphicStyleStylizedV2,
    #[serde(rename = "vaf_games_happy")]
    GamesHappy,
    #[serde(rename = "vaf_games_intense")]
    GamesIntense,
    #[serde(rename = "vaf_games_interactive")]
    GamesInteractive,
    #[serde(rename = "vaf_games_massively_multi_player")]
    GamesMassivelyMultiPlayer,
    #[serde(rename = "vaf_games_multi_player")]
    GamesMultiPlayer,
    #[serde(rename = "vaf_games_multi_player_friends")]
    GamesMultiPlayerFriends,
    #[serde(rename = "vaf_games_multi_player_v1")]
    GamesMultiPlayerV1,
    #[serde(rename = "vaf_games_multi_player_v2")]
    GamesMultiPlayerV2,
    #[serde(rename = "vaf_games_offline_v1")]
    GamesOfflineV1,
    #[serde(rename = "vaf_games_offline_v2")]
    GamesOfflineV2,
    #[serde(rename = "vaf_games_rating_controls")]
    GamesRatingControls,
    #[serde(rename = "vaf_games_rating_gameplay")]
    GamesRatingGameplay,
    #[serde(rename = "vaf_games_rating_graphics")]
    GamesRatingGraphics,
    #[serde(rename = "vaf_games_realistic")]
    GamesRealistic,
    #[serde(rename = "vaf_games_relaxing")]
    GamesRelaxing,
    #[serde(rename = "vaf_games_sad")]
    GamesSad,
    #[serde(rename = "vaf_games_scary")]
    GamesScary,
    #[serde(rename = "vaf_games_silly")]
    GamesSilly,
    #[serde(rename = "vaf_games_simple")]
    GamesSimple,
    #[serde(rename = "vaf_games_single_player")]
    GamesSinglePlayer,
    #[serde(rename = "vaf_games_smooth")]
    GamesSmooth,
    #[serde(rename = "vaf_games_subject_archery")]
    GamesSubjectArchery,
    #[serde(rename = "vaf_games_subject_army")]
    GamesSubjectArmy,
    #[serde(rename = "vaf_games_subject_bicycle")]
    GamesSubjectBicycle,
    #[serde(rename = "vaf_games_subject_bmx")]
    GamesSubjectBmx,
    #[serde(rename = "vaf_games_subject_boating")]
    GamesSubjectBoating,
    #[serde(rename = "vaf_games_subject_bowling")]
    GamesSubjectBowling,
    #[serde(rename = "vaf_games_subject_breeding")]
    GamesSubjectBreeding,
    #[serde(rename = "vaf_games_subject_car")]
    GamesSubjectCar,
    #[serde(rename = "vaf_games_subject_chess")]
    GamesSubjectChess,
    #[serde(rename = "vaf_games_subject_cooking")]
    GamesSubjectCooking,
    #[serde(rename = "vaf_games_subject_dance")]
    GamesSubjectDance,
    #[serde(rename = "vaf_games_subject_dentist")]
    GamesSubjectDentist,
    #[serde(rename = "vaf_games_subject_doctor")]
    GamesSubjectDoctor,
    #[serde(rename = "vaf_games_subject_domino")]
    GamesSubjectDomino,
    #[serde(rename = "vaf_games_subject_drag_racing")]
    GamesSubjectDragRacing,
    #[serde(rename = "vaf_games_subject_drift_racing")]
    GamesSubjectDriftRacing,
    #[serde(rename = "vaf_games_subject_farm")]
    GamesSubjectFarm,
    #[serde(rename = "vaf_games_subject_flight")]
    GamesSubjectFlight,
    #[serde(rename = "vaf_games_subject_go")]
    GamesSubjectGo,
    #[serde(rename = "vaf_games_subject_hunting")]
    GamesSubjectHunting,
    #[serde(rename = "vaf_games_subject_instrument")]
    GamesSubjectInstrument,
    #[serde(rename = "vaf_games_subject_karate")]
    GamesSubjectKarate,
    #[serde(rename = "vaf_games_subject_kart")]
    GamesSubjectKart,
    #[serde(rename = "vaf_games_subject_ninja")]
    GamesSubjectNinja,
    #[serde(rename = "vaf_games_subject_prank")]
    GamesSubjectPrank,
    #[serde(rename = "vaf_games_subject_racket_sports")]
    GamesSubjectRacketSports,
    #[serde(rename = "vaf_games_subject_rally_racing")]
    GamesSubjectRallyRacing,
    #[serde(rename = "vaf_games_subject_skateboarding")]
    GamesSubjectSkateboarding,
    #[serde(rename = "vaf_games_subject_sniper")]
    GamesSubjectSniper,
    #[serde(rename = "vaf_games_subject_snowboarding")]
    GamesSubjectSnowboarding,
    #[serde(rename = "vaf_games_subject_solitaire")]
    GamesSubjectSolitaire,
    #[serde(rename = "vaf_games_subject_space_simulation")]
    GamesSubjectSpaceSimulation,
    #[serde(rename = "vaf_games_subject_spades")]
    GamesSubjectSpades,
    #[serde(rename = "vaf_games_subject_stick_figure")]
    GamesSubjectStickFigure,
    #[serde(rename = "vaf_games_subject_submarine")]
    GamesSubjectSubmarine,
    #[serde(rename = "vaf_games_subject_superhero")]
    GamesSubjectSuperhero,
    #[serde(rename = "vaf_games_subject_surfing")]
    GamesSubjectSurfing,
    #[serde(rename = "vaf_games_subject_talking_animals")]
    GamesSubjectTalkingAnimals,
    #[serde(rename = "vaf_games_subject_tank")]
    GamesSubjectTank,
    #[serde(rename = "vaf_games_subject_train")]
    GamesSubjectTrain,
    #[serde(rename = "vaf_games_subject_truck")]
    GamesSubjectTruck,
    #[serde(rename = "vaf_games_subject_vehicle_simulation")]
    GamesSubjectVehicleSimulation,
    #[serde(rename = "vaf_games_subject_zombie")]
    GamesSubjectZombie,
    #[serde(rename = "vaf_games_works_offline")]
    GamesWorksOffline,
    #[serde(rename = "vaf_google_cast")]
    GoogleCast,
    #[serde(rename = "vaf_hd_content")]
    HdContent,
    #[serde(rename = "vaf_hd_movies_shows")]
    HdMoviesShows,
    #[serde(rename = "vaf_international_transfers")]
    InternationalTransfers,
    #[serde(rename = "vaf_investing")]
    Investing,
    #[serde(rename = "vaf_learning_piano")]
    LearningPiano,
    #[serde(rename = "vaf_making_collages")]
    MakingCollages,
    #[serde(rename = "vaf_making_scrapbooks")]
    MakingScrapbooks,
    #[serde(rename = "vaf_making_videos")]
    MakingVideos,
    #[serde(rename = "vaf_making_watchlists")]
    MakingWatchlists,
    #[serde(rename = "vaf_map_searches")]
    MapSearches,
    #[serde(rename = "vaf_mark_message_as_read")]
    MarkMessageAsRead,
    #[serde(rename = "vaf_movie_show_info")]
    MovieShowInfo,
    #[serde(rename = "vaf_multi_device_sync")]
    MultiDeviceSync,
    #[serde(rename = "vaf_multiple_languages")]
    MultipleLanguages,
    #[serde(rename = "vaf_never_display_ads_experience")]
    NeverDisplayAdsExperience,
    #[serde(rename = "vaf_never_display_app_description")]
    NeverDisplayAppDescription,
    #[serde(rename = "vaf_never_display_considerate_of_time")]
    NeverDisplayConsiderateOfTime,
    #[serde(rename = "vaf_never_display_content_language")]
    NeverDisplayContentLanguage,
    #[serde(rename = "vaf_never_display_deceptive_ads")]
    NeverDisplayDeceptiveAds,
    #[serde(rename = "vaf_never_display_difficult_to_use")]
    NeverDisplayDifficultToUse,
    #[serde(rename = "vaf_never_display_disruptive_ads")]
    NeverDisplayDisruptiveAds,
    #[serde(rename = "vaf_never_display_ease_of_use")]
    NeverDisplayEaseOfUse,
    #[serde(rename = "vaf_never_display_easy_to_use")]
    NeverDisplayEasyToUse,
    #[serde(rename = "vaf_never_display_inappropriate_ads")]
    NeverDisplayInappropriateAds,
    #[serde(rename = "vaf_never_display_make_you_happy")]
    NeverDisplayMakeYouHappy,
    #[serde(rename = "vaf_never_display_make_you_happy_games")]
    NeverDisplayMakeYouHappyGames,
    #[serde(rename = "vaf_never_display_meet_expectations")]
    NeverDisplayMeetExpectations,
    #[serde(rename = "vaf_never_display_negative_ads_impact")]
    NeverDisplayNegativeAdsImpact,
    #[serde(rename = "vaf_never_display_positive_impact")]
    NeverDisplayPositiveImpact,
    #[serde(rename = "vaf_never_display_spend_more_time")]
    NeverDisplaySpendMoreTime,
    #[serde(rename = "vaf_never_display_useful_notifications")]
    NeverDisplayUsefulNotifications,
    #[serde(rename = "vaf_never_display_visually_bad")]
    NeverDisplayVisuallyBad,
    #[serde(rename = "vaf_never_display_visually_good")]
    NeverDisplayVisuallyGood,
    #[serde(rename = "vaf_personalization")]
    Personalization,
    #[serde(rename = "vaf_phase1_2_step_verification")]
    Phase12StepVerification,
    #[serde(rename = "vaf_phase1_2nd_screen_experience")]
    Phase12ndScreenExperience,
    #[serde(rename = "vaf_phase1_3rd_party_music")]
    Phase13rdPartyMusic,
    #[serde(rename = "vaf_phase1_4k_content")]
    Phase14kContent,
    #[serde(rename = "vaf_phase1_4k_movies_shows")]
    Phase14kMoviesShows,
    #[serde(rename = "vaf_phase1_ability_to_recommend")]
    Phase1AbilityToRecommend,
    #[serde(rename = "vaf_phase1_actor_bios")]
    Phase1ActorBios,
    #[serde(rename = "vaf_phase1_adding_friends_by_qr_code")]
    Phase1AddingFriendsByQrCode,
    #[serde(rename = "vaf_phase1_android_tv")]
    Phase1AndroidTv,
    #[serde(rename = "vaf_phase1_android_wear")]
    Phase1AndroidWear,
    #[serde(rename = "vaf_phase1_android_wear_support")]
    Phase1AndroidWearSupport,
    #[serde(rename = "vaf_phase1_annotating_content")]
    Phase1AnnotatingContent,
    #[serde(rename = "vaf_phase1_auto_investing")]
    Phase1AutoInvesting,
    #[serde(rename = "vaf_phase1_auto_play")]
    Phase1AutoPlay,
    #[serde(rename = "vaf_phase1_autocorrect_feature")]
    Phase1AutocorrectFeature,
    #[serde(rename = "vaf_phase1_available_social_networks")]
    Phase1AvailableSocialNetworks,
    #[serde(rename = "vaf_phase1_backing_up_photos")]
    Phase1BackingUpPhotos,
    #[serde(rename = "vaf_phase1_blur_feature")]
    Phase1BlurFeature,
    #[serde(rename = "vaf_phase1_bokeh_feature")]
    Phase1BokehFeature,
    #[serde(rename = "vaf_phase1_book_reviews")]
    Phase1BookReviews,
    #[serde(rename = "vaf_phase1_bookmarking_content")]
    Phase1BookmarkingContent,
    #[serde(rename = "vaf_phase1_budgeting")]
    Phase1Budgeting,
    #[serde(rename = "vaf_phase1_buying_movies_shows")]
    Phase1BuyingMoviesShows,
    #[serde(rename = "vaf_phase1_buying_photos")]
    Phase1BuyingPhotos,
    #[serde(rename = "vaf_phase1_buying_tickets")]
    Phase1BuyingTickets,
    #[serde(rename = "vaf_phase1_calendar_support")]
    Phase1CalendarSupport,
    #[serde(rename = "vaf_phase1_camera")]
    Phase1Camera,
    #[serde(rename = "vaf_phase1_cardio_training")]
    Phase1CardioTraining,
    #[serde(rename = "vaf_phase1_catalog_of_books")]
    Phase1CatalogOfBooks,
    #[serde(rename = "vaf_phase1_closed_captions")]
    Phase1ClosedCaptions,
    #[serde(rename = "vaf_phase1_collaboration")]
    Phase1Collaboration,
    #[serde(rename = "vaf_phase1_collaborative_editing")]
    Phase1CollaborativeEditing,
    #[serde(rename = "vaf_phase1_commenting_on_books")]
    Phase1CommentingOnBooks,
    #[serde(rename = "vaf_phase1_comparing_prices")]
    Phase1ComparingPrices,
    #[serde(rename = "vaf_phase1_creating_albums")]
    Phase1CreatingAlbums,
    #[serde(rename = "vaf_phase1_creating_gifs")]
    Phase1CreatingGifs,
    #[serde(rename = "vaf_phase1_creating_routes")]
    Phase1CreatingRoutes,
    #[serde(rename = "vaf_phase1_creating_watchlists")]
    Phase1CreatingWatchlists,
    #[serde(rename = "vaf_phase1_critics__reviews")]
    Phase1CriticsReviews,
    #[serde(rename = "vaf_phase1_cropping_photos")]
    Phase1CroppingPhotos,
    #[serde(rename = "vaf_phase1_custom_controls")]
    Phase1CustomControls,
    #[serde(rename = "vaf_phase1_device_mirroring")]
    Phase1DeviceMirroring,
    #[serde(rename = "vaf_phase1_dictionary")]
    Phase1Dictionary,
    #[serde(rename = "vaf_phase1_dj_feature")]
    Phase1DjFeature,
    #[serde(rename = "vaf_phase1_downloading_podcasts")]
    Phase1DownloadingPodcasts,
    #[serde(rename = "vaf_phase1_editing_photos")]
    Phase1EditingPhotos,
    #[serde(rename = "vaf_phase1_editing_ringtones")]
    Phase1EditingRingtones,
    #[serde(rename = "vaf_phase1_editing_text")]
    Phase1EditingText,
    #[serde(rename = "vaf_phase1_emojis")]
    Phase1Emojis,
    #[serde(rename = "vaf_phase1_equalizer")]
    Phase1Equalizer,
    #[serde(rename = "vaf_phase1_fdic_insured")]
    Phase1FdicInsured,
    #[serde(rename = "vaf_phase1_file_sharing")]
    Phase1FileSharing,
    #[serde(rename = "vaf_phase1_filmographies")]
    Phase1Filmographies,
    #[serde(rename = "vaf_phase1_filters")]
    Phase1Filters,
    #[serde(rename = "vaf_phase1_financial_news")]
    Phase1FinancialNews,
    #[serde(rename = "vaf_phase1_finding_playlists")]
    Phase1FindingPlaylists,
    #[serde(rename = "vaf_phase1_fitness_app_integration")]
    Phase1FitnessAppIntegration,
    #[serde(rename = "vaf_phase1_fitness_devices")]
    Phase1FitnessDevices,
    #[serde(rename = "vaf_phase1_free_communication")]
    Phase1FreeCommunication,
    #[serde(rename = "vaf_phase1_free_content")]
    Phase1FreeContent,
    #[serde(rename = "vaf_phase1_free_movies_shows")]
    Phase1FreeMoviesShows,
    #[serde(rename = "vaf_phase1_free_storage")]
    Phase1FreeStorage,
    #[serde(rename = "vaf_phase1_free_trial")]
    Phase1FreeTrial,
    #[serde(rename = "vaf_phase1_getting_credit_scores")]
    Phase1GettingCreditScores,
    #[serde(rename = "vaf_phase1_gif_support")]
    Phase1GifSupport,
    #[serde(rename = "vaf_phase1_google_cast")]
    Phase1GoogleCast,
    #[serde(rename = "vaf_phase1_gps_tracking")]
    Phase1GpsTracking,
    #[serde(rename = "vaf_phase1_guide_exercise")]
    Phase1GuideExercise,
    #[serde(rename = "vaf_phase1_hd_content")]
    Phase1HdContent,
    #[serde(rename = "vaf_phase1_hd_movies_shows")]
    Phase1HdMoviesShows,
    #[serde(rename = "vaf_phase1_highlighting_text")]
    Phase1HighlightingText,
    #[serde(rename = "vaf_phase1_homescreen_widgets")]
    Phase1HomescreenWidgets,
    #[serde(rename = "vaf_phase1_identifying_songs")]
    Phase1IdentifyingSongs,
    #[serde(rename = "vaf_phase1_importing_recipes")]
    Phase1ImportingRecipes,
    #[serde(rename = "vaf_phase1_inviting_friends")]
    Phase1InvitingFriends,
    #[serde(rename = "vaf_phase1_leaderboards")]
    Phase1Leaderboards,
    #[serde(rename = "vaf_phase1_learning_drums")]
    Phase1LearningDrums,
    #[serde(rename = "vaf_phase1_learning_guitar")]
    Phase1LearningGuitar,
    #[serde(rename = "vaf_phase1_learning_piano")]
    Phase1LearningPiano,
    #[serde(rename = "vaf_phase1_learning_to_sing")]
    Phase1LearningToSing,
    #[serde(rename = "vaf_phase1_lending_books_to_friends")]
    Phase1LendingBooksToFriends,
    #[serde(rename = "vaf_phase1_life_insurance")]
    Phase1LifeInsurance,
    #[serde(rename = "vaf_phase1_listening_offline")]
    Phase1ListeningOffline,
    #[serde(rename = "vaf_phase1_live_channels")]
    Phase1LiveChannels,
    #[serde(rename = "vaf_phase1_location_tagging")]
    Phase1LocationTagging,
    #[serde(rename = "vaf_phase1_lockscreen_display")]
    Phase1LockscreenDisplay,
    #[serde(rename = "vaf_phase1_loyalty_program")]
    Phase1LoyaltyProgram,
    #[serde(rename = "vaf_phase1_making_collages")]
    Phase1MakingCollages,
    #[serde(rename = "vaf_phase1_making_lists")]
    Phase1MakingLists,
    #[serde(rename = "vaf_phase1_making_playlists")]
    Phase1MakingPlaylists,
    #[serde(rename = "vaf_phase1_making_ringtones")]
    Phase1MakingRingtones,
    #[serde(rename = "vaf_phase1_making_scrapbooks")]
    Phase1MakingScrapbooks,
    #[serde(rename = "vaf_phase1_making_videos")]
    Phase1MakingVideos,
    #[serde(rename = "vaf_phase1_making_voip_calls")]
    Phase1MakingVoipCalls,
    #[serde(rename = "vaf_phase1_making_watchlists")]
    Phase1MakingWatchlists,
    #[serde(rename = "vaf_phase1_managing_events")]
    Phase1ManagingEvents,
    #[serde(rename = "vaf_phase1_map_searches")]
    Phase1MapSearches,
    #[serde(rename = "vaf_phase1_messaging")]
    Phase1Messaging,
    #[serde(rename = "vaf_phase1_midi_support")]
    Phase1MidiSupport,
    #[serde(rename = "vaf_phase1_monitoring_credit")]
    Phase1MonitoringCredit,
    #[serde(rename = "vaf_phase1_movie_show_info")]
    Phase1MovieShowInfo,
    #[serde(rename = "vaf_phase1_multi_device_sync")]
    Phase1MultiDeviceSync,
    #[serde(rename = "vaf_phase1_multi_language_support")]
    Phase1MultiLanguageSupport,
    #[serde(rename = "vaf_phase1_multiple_languages")]
    Phase1MultipleLanguages,
    #[serde(rename = "vaf_phase1_night_reading_mode")]
    Phase1NightReadingMode,
    #[serde(rename = "vaf_phase1_nutrition_plans")]
    Phase1NutritionPlans,
    #[serde(rename = "vaf_phase1_offline_logging")]
    Phase1OfflineLogging,
    #[serde(rename = "vaf_phase1_offline_viewing")]
    Phase1OfflineViewing,
    #[serde(rename = "vaf_phase1_online_storage")]
    Phase1OnlineStorage,
    #[serde(rename = "vaf_phase1_organizational_tools")]
    Phase1OrganizationalTools,
    #[serde(rename = "vaf_phase1_paid_stickers")]
    Phase1PaidStickers,
    #[serde(rename = "vaf_phase1_period_tracking")]
    Phase1PeriodTracking,
    #[serde(rename = "vaf_phase1_personal_loans")]
    Phase1PersonalLoans,
    #[serde(rename = "vaf_phase1_personalization")]
    Phase1Personalization,
    #[serde(rename = "vaf_phase1_photo_contests")]
    Phase1PhotoContests,
    #[serde(rename = "vaf_phase1_photo_framing")]
    Phase1PhotoFraming,
    #[serde(rename = "vaf_phase1_planning_dates")]
    Phase1PlanningDates,
    #[serde(rename = "vaf_phase1_playing_movies")]
    Phase1PlayingMovies,
    #[serde(rename = "vaf_phase1_podcasts")]
    Phase1Podcasts,
    #[serde(rename = "vaf_phase1_predictive_typing")]
    Phase1PredictiveTyping,
    #[serde(rename = "vaf_phase1_printing_photos")]
    Phase1PrintingPhotos,
    #[serde(rename = "vaf_phase1_privacy_controls")]
    Phase1PrivacyControls,
    #[serde(rename = "vaf_phase1_questionnaires")]
    Phase1Questionnaires,
    #[serde(rename = "vaf_phase1_reading_bios")]
    Phase1ReadingBios,
    #[serde(rename = "vaf_phase1_reading_offline")]
    Phase1ReadingOffline,
    #[serde(rename = "vaf_phase1_real_time_following")]
    Phase1RealTimeFollowing,
    #[serde(rename = "vaf_phase1_recommendations")]
    Phase1Recommendations,
    #[serde(rename = "vaf_phase1_rentals")]
    Phase1Rentals,
    #[serde(rename = "vaf_phase1_renting_movies_shows")]
    Phase1RentingMoviesShows,
    #[serde(rename = "vaf_phase1_reposting_photos")]
    Phase1RepostingPhotos,
    #[serde(rename = "vaf_phase1_resume_watching")]
    Phase1ResumeWatching,
    #[serde(rename = "vaf_phase1_resuming_viewing")]
    Phase1ResumingViewing,
    #[serde(rename = "vaf_phase1_reviews_by_critics")]
    Phase1ReviewsByCritics,
    #[serde(rename = "vaf_phase1_ringtones")]
    Phase1Ringtones,
    #[serde(rename = "vaf_phase1_rotating_photos")]
    Phase1RotatingPhotos,
    #[serde(rename = "vaf_phase1_samples")]
    Phase1Samples,
    #[serde(rename = "vaf_phase1_screensharing")]
    Phase1Screensharing,
    #[serde(rename = "vaf_phase1_search_messages")]
    Phase1SearchMessages,
    #[serde(rename = "vaf_phase1_see_your_posting_history")]
    Phase1SeeYourPostingHistory,
    #[serde(rename = "vaf_phase1_selfies")]
    Phase1Selfies,
    #[serde(rename = "vaf_phase1_sending_digital_gifts")]
    Phase1SendingDigitalGifts,
    #[serde(rename = "vaf_phase1_setting_goals")]
    Phase1SettingGoals,
    #[serde(rename = "vaf_phase1_setting_reminders")]
    Phase1SettingReminders,
    #[serde(rename = "vaf_phase1_sharing_arrival_time")]
    Phase1SharingArrivalTime,
    #[serde(rename = "vaf_phase1_sharing_photos")]
    Phase1SharingPhotos,
    #[serde(rename = "vaf_phase1_sharing_songs")]
    Phase1SharingSongs,
    #[serde(rename = "vaf_phase1_showing_old_memories")]
    Phase1ShowingOldMemories,
    #[serde(rename = "vaf_phase1_showtimes")]
    Phase1Showtimes,
    #[serde(rename = "vaf_phase1_slideshows")]
    Phase1Slideshows,
    #[serde(rename = "vaf_phase1_smart_scales")]
    Phase1SmartScales,
    #[serde(rename = "vaf_phase1_smartwatch_support")]
    Phase1SmartwatchSupport,
    #[serde(rename = "vaf_phase1_social_features")]
    Phase1SocialFeatures,
    #[serde(rename = "vaf_phase1_social_news_feed")]
    Phase1SocialNewsFeed,
    #[serde(rename = "vaf_phase1_social_recommendations")]
    Phase1SocialRecommendations,
    #[serde(rename = "vaf_phase1_social_sign_in")]
    Phase1SocialSignIn,
    #[serde(rename = "vaf_phase1_song_lyrics")]
    Phase1SongLyrics,
    #[serde(rename = "vaf_phase1_song_recommendations")]
    Phase1SongRecommendations,
    #[serde(rename = "vaf_phase1_sound_editor")]
    Phase1SoundEditor,
    #[serde(rename = "vaf_phase1_sound_mixer")]
    Phase1SoundMixer,
    #[serde(rename = "vaf_phase1_sound_recording")]
    Phase1SoundRecording,
    #[serde(rename = "vaf_phase1_specific_communities")]
    Phase1SpecificCommunities,
    #[serde(rename = "vaf_phase1_stickers")]
    Phase1Stickers,
    #[serde(rename = "vaf_phase1_streaming_music")]
    Phase1StreamingMusic,
    #[serde(rename = "vaf_phase1_strength_training")]
    Phase1StrengthTraining,
    #[serde(rename = "vaf_phase1_submitting_content")]
    Phase1SubmittingContent,
    #[serde(rename = "vaf_phase1_subscriptions")]
    Phase1Subscriptions,
    #[serde(rename = "vaf_phase1_syncing_with_devices")]
    Phase1SyncingWithDevices,
    #[serde(rename = "vaf_phase1_taking_notes")]
    Phase1TakingNotes,
    #[serde(rename = "vaf_phase1_track_fitness_activities")]
    Phase1TrackFitnessActivities,
    #[serde(rename = "vaf_phase1_tracking_finances")]
    Phase1TrackingFinances,
    #[serde(rename = "vaf_phase1_tracking_stocks")]
    Phase1TrackingStocks,
    #[serde(rename = "vaf_phase1_tracking_weight")]
    Phase1TrackingWeight,
    #[serde(rename = "vaf_phase1_tracking_workouts")]
    Phase1TrackingWorkouts,
    #[serde(rename = "vaf_phase1_training_community")]
    Phase1TrainingCommunity,
    #[serde(rename = "vaf_phase1_training_plans")]
    Phase1TrainingPlans,
    #[serde(rename = "vaf_phase1_uploading_photos")]
    Phase1UploadingPhotos,
    #[serde(rename = "vaf_phase1_uploading_videos")]
    Phase1UploadingVideos,
    #[serde(rename = "vaf_phase1_user_community")]
    Phase1UserCommunity,
    #[serde(rename = "vaf_phase1_user_reviews")]
    Phase1UserReviews,
    #[serde(rename = "vaf_phase1_variety_of_backgrounds")]
    Phase1VarietyOfBackgrounds,
    #[serde(rename = "vaf_phase1_verbal_feedback")]
    Phase1VerbalFeedback,
    #[serde(rename = "vaf_phase1_video_chat")]
    Phase1VideoChat,
    #[serde(rename = "vaf_phase1_video_group_messaging")]
    Phase1VideoGroupMessaging,
    #[serde(rename = "vaf_phase1_virtual_reality")]
    Phase1VirtualReality,
    #[serde(rename = "vaf_phase1_voice_coach")]
    Phase1VoiceCoach,
    #[serde(rename = "vaf_phase1_voice_controls")]
    Phase1VoiceControls,
    #[serde(rename = "vaf_phase1_voice_messaging")]
    Phase1VoiceMessaging,
    #[serde(rename = "vaf_phase1_vr_support")]
    Phase1VrSupport,
    #[serde(rename = "vaf_phase1_wallpapers")]
    Phase1Wallpapers,
    #[serde(rename = "vaf_phase1_watching_live")]
    Phase1WatchingLive,
    #[serde(rename = "vaf_phase1_watching_offline")]
    Phase1WatchingOffline,
    #[serde(rename = "vaf_phase1_watching_trailers")]
    Phase1WatchingTrailers,
    #[serde(rename = "vaf_phase1_whiteboarding")]
    Phase1Whiteboarding,
    #[serde(rename = "vaf_phase1_widgets")]
    Phase1Widgets,
    #[serde(rename = "vaf_phase1_working_offline")]
    Phase1WorkingOffline,
    #[serde(rename = "vaf_phase1_workout_diary")]
    Phase1WorkoutDiary,
    #[serde(rename = "vaf_phase1_works_offline")]
    Phase1WorksOffline,
    #[serde(rename = "vaf_photo_contests")]
    PhotoContests,
    #[serde(rename = "vaf_photo_framing")]
    PhotoFraming,
    #[serde(rename = "vaf_playing_movies")]
    PlayingMovies,
    #[serde(rename = "vaf_printing_photos")]
    PrintingPhotos,
    #[serde(rename = "vaf_reading_bios")]
    ReadingBios,
    #[serde(rename = "vaf_recommendations")]
    Recommendations,
    #[serde(rename = "vaf_rentals")]
    Rentals,
    #[serde(rename = "vaf_resume_watching")]
    ResumeWatching,
    #[serde(rename = "vaf_resuming_viewing")]
    ResumingViewing,
    #[serde(rename = "vaf_reviews_by_critics")]
    ReviewsByCritics,
    #[serde(rename = "vaf_ringtones")]
    Ringtones,
    #[serde(rename = "vaf_rotating_photos")]
    RotatingPhotos,
    #[serde(rename = "vaf_seeing_photos_of_places")]
    SeeingPhotosOfPlaces,
    #[serde(rename = "vaf_sharing_photos")]
    SharingPhotos,
    #[serde(rename = "vaf_showing_old_memories")]
    ShowingOldMemories,
    #[serde(rename = "vaf_slideshows")]
    Slideshows,
    #[serde(rename = "vaf_social_features")]
    SocialFeatures,
    #[serde(rename = "vaf_song_lyrics")]
    SongLyrics,
    #[serde(rename = "vaf_sound_recording")]
    SoundRecording,
    #[serde(rename = "vaf_stickers")]
    Stickers,
    #[serde(rename = "vaf_taking_photos")]
    TakingPhotos,
    #[serde(rename = "vaf_track_fitness_activities")]
    TrackFitnessActivities,
    #[serde(rename = "vaf_tracking_finances")]
    TrackingFinances,
    #[serde(rename = "vaf_tracking_workouts")]
    TrackingWorkouts,
    #[serde(rename = "vaf_training_plans")]
    TrainingPlans,
    #[serde(rename = "vaf_user_community")]
    UserCommunity,
    #[serde(rename = "vaf_user_reviews")]
    UserReviews,
    #[serde(rename = "vaf_virtual_reality")]
    VirtualReality,
    #[serde(rename = "vaf_wallpapers")]
    Wallpapers,
    #[serde(rename = "vaf_watching_offline")]
    WatchingOffline,
    #[serde(rename = "vaf_watching_trailers")]
    WatchingTrailers,
    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Other(String),
}

impl std::str::FromStr for CriterionType {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde::de::Deserialize::deserialize(serde::de::IntoDeserializer::into_deserializer(s))
    }
}
impl std::fmt::Display for CriterionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde::Serialize::serialize(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::CriterionType;

    #[test]
    fn criterion_type_order_is_alphabetical_by_rename() {
        let values = all_variants();
        let renames = values
            .into_iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>();
        let mut sorted_renames = renames.clone();
        sorted_renames.sort();

        assert_eq!(
            renames, sorted_renames,
            "CriterionType variants are not ordered alphabetically by their rename values"
        );
    }

    /// Exhaustive list of all variants (needed because Rust doesn't provide this automatically).
    /// You already have them declared, so we just collect them here.
    fn all_variants() -> Vec<CriterionType> {
        use CriterionType::*;
        let mut variants = vec![
            SecondScreenExperience,
            FourKContent,
            FourKMoviesShows,
            ActorBios,
            AgeTargeting68,
            AndroidTv,
            AppsAttributeAnimeV1,
            AppsAttributeAnimeV2,
            AppsAttributeDownloadForOfflineViewingV1,
            AppsAttributeDownloadForOfflineViewingV2,
            AppsAttributeFaceSwapV1,
            AppsAttributeFaceSwapV2,
            AppsAttributeLiveV1,
            AppsAttributeLiveV2,
            AppsAttributeSportsFeaturesOutOfNetworkV1,
            AppsAttributeSportsFeaturesOutOfNetworkV2,
            AppsCategoryFoodAndDrinkV1,
            AppsCategoryFoodAndDrinkV2,
            AppsCategoryRestaurantV1,
            AppsCategoryRestaurantV2,
            AutoPlay,
            AvailableSocialNetworks,
            BackingUpPhotos,
            BlurFeature,
            BuyingMoviesShows,
            BuyingTickets,
            Camera,
            CancellationPolicy,
            ChangingReservations,
            ClosedCaptions,
            CreatingAlbums,
            CreatingGifs,
            CroppingPhotos,
            DeleteMessages,
            Dictionary,
            EditingPhotos,
            EditingText,
            Emojis,
            Equalizer,
            Filmographies,
            Filters,
            FreeContent,
            FreeStorage,
            FreeTrial,
            GamesAddictive,
            GamesBeautiful,
            GamesChallenging,
            GamesClassic,
            GamesComplex,
            GamesConnectivityRequiredBeforeFirstUse,
            GamesCustomizable,
            GamesCute,
            GamesDimensionalPlane3dV1,
            GamesDimensionalPlane3dV2,
            GamesEducational,
            GamesEntertaining,
            GamesExciting,
            GamesFeatureLeaderboard,
            GamesFun,
            GamesFunny,
            GamesGenre4xExploreExpandExploitAndExterminate,
            GamesGenreAbstractStrategy,
            GamesGenreAction,
            GamesGenreActionAdventure,
            GamesGenreActionRpg,
            GamesGenreAdventure,
            GamesGenreArcade,
            GamesGenreArtillery,
            GamesGenreBall,
            GamesGenreBaseBuilding,
            GamesGenreBoard,
            GamesGenreBrainTeaser,
            GamesGenreBrainTraining,
            GamesGenreBubbleShooter,
            GamesGenreCard,
            GamesGenreCasual,
            GamesGenreCityBuilding,
            GamesGenreClaw,
            GamesGenreConstructionAndManagementSimulation,
            GamesGenreCrosswords,
            GamesGenreDatingSimulation,
            GamesGenreDice,
            GamesGenreDressUpAndFashion,
            GamesGenreDungeonCrawlers,
            GamesGenreEmpireBuilding,
            GamesGenreEndlessRunners,
            GamesGenreExploration,
            GamesGenreFantasyRpg,
            GamesGenreFighting,
            GamesGenreFirstActionAdventure,
            GamesGenreFirstPersonShooter,
            GamesGenreGovernmentSimulation,
            GamesGenreGraphicAdventure,
            GamesGenreGuessing,
            GamesGenreHackAndSlash,
            GamesGenreHeroShooter,
            GamesGenreHorror,
            GamesGenreIncremental,
            GamesGenreInteractiveFiction,
            GamesGenreIsometricPlatform,
            GamesGenreJapaneseRpg,
            GamesGenreJigsaw,
            GamesGenreLifeSimulation,
            GamesGenreLogic,
            GamesGenreMassivelyMultiplayerOnline,
            GamesGenreMassivelyMultiplayerOnlineFirstPersonShooter,
            GamesGenreMassivelyMultiplayerOnlineRolePlaying,
            GamesGenreMaze,
            GamesGenreMedicalSimulation,
            GamesGenreMemory,
            GamesGenreMilitaryStrategy,
            GamesGenreMinigames,
            GamesGenreMultiplayerOnlineBattleArena,
            GamesGenreMysteryAdventure,
            GamesGenrePlatformAdventure,
            GamesGenrePlatformers,
            GamesGenrePuzzle,
            GamesGenrePuzzleV1,
            GamesGenrePuzzleV2,
            GamesGenreRacing,
            GamesGenreRealTimeStrategy,
            GamesGenreRealTimeTactics,
            GamesGenreRestaurant,
            GamesGenreRevealThePicture,
            GamesGenreRhythm,
            GamesGenreRoguelike,
            GamesGenreRolePlaying,
            GamesGenreRoomEscape,
            GamesGenreRummy,
            GamesGenreSandboxAndOpenWorldRpg,
            GamesGenreScienceFiction,
            GamesGenreShootEmUp,
            GamesGenreShooting,
            GamesGenreShootingGallery,
            GamesGenreSimRacing,
            GamesGenreSimulation,
            GamesGenreSlidingPuzzle,
            GamesGenreSlotMachine,
            GamesGenreSocialSimulation,
            GamesGenreSokoban,
            GamesGenreStealth,
            GamesGenreStrategy,
            GamesGenreStrategyRpg,
            GamesGenreStuntDriving,
            GamesGenreSurvival,
            GamesGenreSurvivalHorror,
            GamesGenreTable,
            GamesGenreTacticalRpg,
            GamesGenreTacticalShooter,
            GamesGenreThirdPersonActionAdventure,
            GamesGenreThirdPersonShooter,
            GamesGenreTileMatching,
            GamesGenreTradingCard,
            GamesGenreTrivia,
            GamesGenreTurnBasedRpgV1,
            GamesGenreTurnBasedRpgV2,
            GamesGenreTurnBasedStrategy,
            GamesGenreTurnBasedTactics,
            GamesGenreTycoon,
            GamesGenreWargames,
            GamesGraphicStyleAnimeV1,
            GamesGraphicStyleAnimeV2,
            GamesGraphicStyleCartoonV1,
            GamesGraphicStyleCartoonV2,
            GamesGraphicStylePixelatedV1,
            GamesGraphicStylePixelatedV2,
            GamesGraphicStyleStylizedV1,
            GamesGraphicStyleStylizedV2,
            GamesHappy,
            GamesIntense,
            GamesInteractive,
            GamesMassivelyMultiPlayer,
            GamesMultiPlayer,
            GamesMultiPlayerFriends,
            GamesMultiPlayerV1,
            GamesMultiPlayerV2,
            GamesOfflineV1,
            GamesOfflineV2,
            GamesRatingControls,
            GamesRatingGameplay,
            GamesRatingGraphics,
            GamesRealistic,
            GamesRelaxing,
            GamesSad,
            GamesScary,
            GamesSilly,
            GamesSimple,
            GamesSinglePlayer,
            GamesSmooth,
            GamesSubjectArchery,
            GamesSubjectArmy,
            GamesSubjectBicycle,
            GamesSubjectBmx,
            GamesSubjectBoating,
            GamesSubjectBowling,
            GamesSubjectBreeding,
            GamesSubjectCar,
            GamesSubjectChess,
            GamesSubjectCooking,
            GamesSubjectDance,
            GamesSubjectDentist,
            GamesSubjectDoctor,
            GamesSubjectDomino,
            GamesSubjectDragRacing,
            GamesSubjectDriftRacing,
            GamesSubjectFarm,
            GamesSubjectFlight,
            GamesSubjectGo,
            GamesSubjectHunting,
            GamesSubjectInstrument,
            GamesSubjectKarate,
            GamesSubjectKart,
            GamesSubjectNinja,
            GamesSubjectPrank,
            GamesSubjectRacketSports,
            GamesSubjectRallyRacing,
            GamesSubjectSkateboarding,
            GamesSubjectSniper,
            GamesSubjectSnowboarding,
            GamesSubjectSolitaire,
            GamesSubjectSpaceSimulation,
            GamesSubjectSpades,
            GamesSubjectStickFigure,
            GamesSubjectSubmarine,
            GamesSubjectSuperhero,
            GamesSubjectSurfing,
            GamesSubjectTalkingAnimals,
            GamesSubjectTank,
            GamesSubjectTrain,
            GamesSubjectTruck,
            GamesSubjectVehicleSimulation,
            GamesSubjectZombie,
            GamesWorksOffline,
            GoogleCast,
            HdContent,
            HdMoviesShows,
            InternationalTransfers,
            Investing,
            LearningPiano,
            MakingCollages,
            MakingScrapbooks,
            MakingVideos,
            MakingWatchlists,
            MapSearches,
            MarkMessageAsRead,
            MovieShowInfo,
            MultiDeviceSync,
            MultipleLanguages,
            NeverDisplayAdsExperience,
            NeverDisplayAppDescription,
            NeverDisplayConsiderateOfTime,
            NeverDisplayContentLanguage,
            NeverDisplayDeceptiveAds,
            NeverDisplayDifficultToUse,
            NeverDisplayDisruptiveAds,
            NeverDisplayEaseOfUse,
            NeverDisplayEasyToUse,
            NeverDisplayInappropriateAds,
            NeverDisplayMakeYouHappy,
            NeverDisplayMakeYouHappyGames,
            NeverDisplayMeetExpectations,
            NeverDisplayNegativeAdsImpact,
            NeverDisplayPositiveImpact,
            NeverDisplaySpendMoreTime,
            NeverDisplayUsefulNotifications,
            NeverDisplayVisuallyBad,
            NeverDisplayVisuallyGood,
            Personalization,
            Phase12StepVerification,
            Phase12ndScreenExperience,
            Phase13rdPartyMusic,
            Phase14kContent,
            Phase14kMoviesShows,
            Phase1AbilityToRecommend,
            Phase1ActorBios,
            Phase1AddingFriendsByQrCode,
            Phase1AndroidTv,
            Phase1AndroidWear,
            Phase1AndroidWearSupport,
            Phase1AnnotatingContent,
            Phase1AutoInvesting,
            Phase1AutoPlay,
            Phase1AutocorrectFeature,
            Phase1AvailableSocialNetworks,
            Phase1BackingUpPhotos,
            Phase1BlurFeature,
            Phase1BokehFeature,
            Phase1BookReviews,
            Phase1BookmarkingContent,
            Phase1Budgeting,
            Phase1BuyingMoviesShows,
            Phase1BuyingPhotos,
            Phase1BuyingTickets,
            Phase1CalendarSupport,
            Phase1Camera,
            Phase1CardioTraining,
            Phase1CatalogOfBooks,
            Phase1ClosedCaptions,
            Phase1Collaboration,
            Phase1CollaborativeEditing,
            Phase1CommentingOnBooks,
            Phase1ComparingPrices,
            Phase1CreatingAlbums,
            Phase1CreatingGifs,
            Phase1CreatingRoutes,
            Phase1CreatingWatchlists,
            Phase1CriticsReviews,
            Phase1CroppingPhotos,
            Phase1CustomControls,
            Phase1DeviceMirroring,
            Phase1Dictionary,
            Phase1DjFeature,
            Phase1DownloadingPodcasts,
            Phase1EditingPhotos,
            Phase1EditingRingtones,
            Phase1EditingText,
            Phase1Emojis,
            Phase1Equalizer,
            Phase1FdicInsured,
            Phase1FileSharing,
            Phase1Filmographies,
            Phase1Filters,
            Phase1FinancialNews,
            Phase1FindingPlaylists,
            Phase1FitnessAppIntegration,
            Phase1FitnessDevices,
            Phase1FreeCommunication,
            Phase1FreeContent,
            Phase1FreeMoviesShows,
            Phase1FreeStorage,
            Phase1FreeTrial,
            Phase1GettingCreditScores,
            Phase1GifSupport,
            Phase1GoogleCast,
            Phase1GpsTracking,
            Phase1GuideExercise,
            Phase1HdContent,
            Phase1HdMoviesShows,
            Phase1HighlightingText,
            Phase1HomescreenWidgets,
            Phase1IdentifyingSongs,
            Phase1ImportingRecipes,
            Phase1InvitingFriends,
            Phase1Leaderboards,
            Phase1LearningDrums,
            Phase1LearningGuitar,
            Phase1LearningPiano,
            Phase1LearningToSing,
            Phase1LendingBooksToFriends,
            Phase1LifeInsurance,
            Phase1ListeningOffline,
            Phase1LiveChannels,
            Phase1LocationTagging,
            Phase1LockscreenDisplay,
            Phase1LoyaltyProgram,
            Phase1MakingCollages,
            Phase1MakingLists,
            Phase1MakingPlaylists,
            Phase1MakingRingtones,
            Phase1MakingScrapbooks,
            Phase1MakingVideos,
            Phase1MakingVoipCalls,
            Phase1MakingWatchlists,
            Phase1ManagingEvents,
            Phase1MapSearches,
            Phase1Messaging,
            Phase1MidiSupport,
            Phase1MonitoringCredit,
            Phase1MovieShowInfo,
            Phase1MultiDeviceSync,
            Phase1MultiLanguageSupport,
            Phase1MultipleLanguages,
            Phase1NightReadingMode,
            Phase1NutritionPlans,
            Phase1OfflineLogging,
            Phase1OfflineViewing,
            Phase1OnlineStorage,
            Phase1OrganizationalTools,
            Phase1PaidStickers,
            Phase1PeriodTracking,
            Phase1PersonalLoans,
            Phase1Personalization,
            Phase1PhotoContests,
            Phase1PhotoFraming,
            Phase1PlanningDates,
            Phase1PlayingMovies,
            Phase1Podcasts,
            Phase1PredictiveTyping,
            Phase1PrintingPhotos,
            Phase1PrivacyControls,
            Phase1Questionnaires,
            Phase1ReadingBios,
            Phase1ReadingOffline,
            Phase1RealTimeFollowing,
            Phase1Recommendations,
            Phase1Rentals,
            Phase1RentingMoviesShows,
            Phase1RepostingPhotos,
            Phase1ResumeWatching,
            Phase1ResumingViewing,
            Phase1ReviewsByCritics,
            Phase1Ringtones,
            Phase1RotatingPhotos,
            Phase1Samples,
            Phase1Screensharing,
            Phase1SearchMessages,
            Phase1SeeYourPostingHistory,
            Phase1Selfies,
            Phase1SendingDigitalGifts,
            Phase1SettingGoals,
            Phase1SettingReminders,
            Phase1SharingArrivalTime,
            Phase1SharingPhotos,
            Phase1SharingSongs,
            Phase1ShowingOldMemories,
            Phase1Showtimes,
            Phase1Slideshows,
            Phase1SmartScales,
            Phase1SmartwatchSupport,
            Phase1SocialFeatures,
            Phase1SocialNewsFeed,
            Phase1SocialRecommendations,
            Phase1SocialSignIn,
            Phase1SongLyrics,
            Phase1SongRecommendations,
            Phase1SoundEditor,
            Phase1SoundMixer,
            Phase1SoundRecording,
            Phase1SpecificCommunities,
            Phase1Stickers,
            Phase1StreamingMusic,
            Phase1StrengthTraining,
            Phase1SubmittingContent,
            Phase1Subscriptions,
            Phase1SyncingWithDevices,
            Phase1TakingNotes,
            Phase1TrackFitnessActivities,
            Phase1TrackingFinances,
            Phase1TrackingStocks,
            Phase1TrackingWeight,
            Phase1TrackingWorkouts,
            Phase1TrainingCommunity,
            Phase1TrainingPlans,
            Phase1UploadingPhotos,
            Phase1UploadingVideos,
            Phase1UserCommunity,
            Phase1UserReviews,
            Phase1VarietyOfBackgrounds,
            Phase1VerbalFeedback,
            Phase1VideoChat,
            Phase1VideoGroupMessaging,
            Phase1VirtualReality,
            Phase1VoiceCoach,
            Phase1VoiceControls,
            Phase1VoiceMessaging,
            Phase1VrSupport,
            Phase1Wallpapers,
            Phase1WatchingLive,
            Phase1WatchingOffline,
            Phase1WatchingTrailers,
            Phase1Whiteboarding,
            Phase1Widgets,
            Phase1WorkingOffline,
            Phase1WorkoutDiary,
            Phase1WorksOffline,
            PhotoContests,
            PhotoFraming,
            PlayingMovies,
            PrintingPhotos,
            ReadingBios,
            Recommendations,
            Rentals,
            ResumeWatching,
            ResumingViewing,
            ReviewsByCritics,
            Ringtones,
            RotatingPhotos,
            SeeingPhotosOfPlaces,
            SharingPhotos,
            ShowingOldMemories,
            Slideshows,
            SocialFeatures,
            SongLyrics,
            SoundRecording,
            Stickers,
            TakingPhotos,
            TrackFitnessActivities,
            TrackingFinances,
            TrackingWorkouts,
            TrainingPlans,
            UserCommunity,
            UserReviews,
            VirtualReality,
            Wallpapers,
            WatchingOffline,
            WatchingTrailers,
        ];

        variants.sort();
        variants
    }
}
