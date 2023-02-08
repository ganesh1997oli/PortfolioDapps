#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod battelship_contract {
    use ink::storage::Mapping;

    #[derive(Debug, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct PlayerState {
        account: AccountId,
        board: [u32; 8],
        shot_x: u32,
        shot_y: u32,
    }

    impl Default for PlayerState {
        fn default() -> Self {
            Self {
                account: zero_addres(),
                board: [0u32; 8],
                shot_x: 0,
                shot_y: 0,
            }
        }
    }

    #[derive(Debug, scale::Decode, scale::Encode, Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct GameState {
        next_turn: u32,
        p1: PlayerState,
        p2: PlayerState,
        last_hit: u8,  // 0 = miss, 1 = hit, 2 = sunk
        sunk_what: u8, // which ship was sunk
    }

    pub type GameId = u32;

    #[ink(storage)]
    pub struct BattelshipContract {
        games: Mapping<GameId, GameState>,
        game_id: u32,
    }

    impl BattelshipContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                games: Mapping::default(),
                game_id: Default::default(),
            }
        }

        #[ink(message)]
        pub fn new_game(&mut self) -> GameState {
            let id = self.game_next_id();
            assert!(self.games.get(id).is_none(), "Game must not exist");

            let player_state = PlayerState {
                account: self.env().caller(),
                board: [0u32; 8],
                shot_x: 0,
                shot_y: 0
            };
            let game_state = GameState {
                next_turn: 0,
                p1: player_state,
                p2: Default::default(),
                last_hit: 0,
                sunk_what: 0,
            };

            self.games.insert(id, &game_state);

            game_state
        }

        #[inline]
        pub fn game_next_id(&mut self) -> GameId {
            let id = self.game_id;
            self.game_id += 1;
            id
        }
    }

    fn zero_addres() -> AccountId {
        [0u8; 32].into()
    }
}
