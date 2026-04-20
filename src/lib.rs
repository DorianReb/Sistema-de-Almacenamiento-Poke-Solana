use anchor_lang::prelude::*;

declare_id!("AkabfcrZ6XsABTxkvbZtynKNndHphU9GUyXULNcWsXCZ");

#[program]
pub mod pc_pokemon {
    use super::*;

    pub fn inicializar_pc(context: Context<NuevaPc>, nombre_entrenador: String) -> Result<()> {
        let owner_id = context.accounts.owner.key(); 
        let equipo: Vec<Pokemon> = Vec::new(); 

        context.accounts.pc_caja.set_inner(PcCaja { 
            owner: owner_id,
            nombre_entrenador,
            equipo,
        });
        Ok(()) 
    }

    pub fn depositar_pokemon(
        context: Context<GestionarPokemon>, 
        especie: String, 
        nivel: u8,
        pokedex_num: u16,
        sexo: String,
        tipos: Vec<String>,
        naturaleza: String,
        habilidad: String
    ) -> Result<()> {
        require!( 
            context.accounts.pc_caja.owner == context.accounts.owner.key(), 
            Errores::NoEsTuPc 
        ); 

        // Obtenemos la fecha actual del reloj de la red (Solana Runtime)
        let clock = Clock::get()?;
        let fecha_encontrado = clock.unix_timestamp;

        let nuevo_pokemon = Pokemon { 
            especie,
            nivel,
            es_shiny: false,
            pokedex_num,
            sexo,
            tipos,
            naturaleza,
            habilidad,
            encontrado_el: fecha_encontrado,
        };

        context.accounts.pc_caja.equipo.push(nuevo_pokemon); 
        Ok(()) 
    }

    pub fn liberar_pokemon(context: Context<GestionarPokemon>, especie: String) -> Result<()> {
        require!( 
            context.accounts.pc_caja.owner == context.accounts.owner.key(),
            Errores::NoEsTuPc
        );

        let equipo = &mut context.accounts.pc_caja.equipo; 
        for i in 0..equipo.len() { 
            if equipo[i].especie == especie { 
                equipo.remove(i);
                return Ok(()); 
            }
        }
        Err(Errores::PokemonNoEncontrado.into()) 
    }

    pub fn alternar_shiny(context: Context<GestionarPokemon>, especie: String) -> Result<()> {
        require!( 
            context.accounts.pc_caja.owner == context.accounts.owner.key(),
            Errores::NoEsTuPc
        );

        let equipo = &mut context.accounts.pc_caja.equipo; 
        for i in 0..equipo.len() { 
            if equipo[i].especie == especie { 
                let estado_actual = equipo[i].es_shiny;
                equipo[i].es_shiny = !estado_actual;
                return Ok(()); 
            }
        }
        Err(Errores::PokemonNoEncontrado.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Acceso denegado: Esta PC pertenece a otro entrenador.")]
    NoEsTuPc,
    #[msg("El Pokémon especificado no se encuentra en esta caja.")]
    PokemonNoEncontrado,
}

#[account]
#[derive(InitSpace)]
pub struct PcCaja { 
    pub owner: Pubkey, 
    #[max_len(40)]
    pub nombre_entrenador: String,
    #[max_len(10)] // Reducimos a 10 para no exceder el límite de memoria por cuenta
    pub equipo: Vec<Pokemon>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Pokemon {
    #[max_len(30)]
    pub especie: String,
    pub nivel: u8, 
    pub es_shiny: bool,
    pub pokedex_num: u16,
    #[max_len(15)] // "Macho", "Hembra", "Desconocido"
    pub sexo: String,
    #[max_len(2, 20)] // Máximo 2 tipos, cada uno de hasta 20 caracteres
    pub tipos: Vec<String>,
    #[max_len(20)]
    pub naturaleza: String,
    #[max_len(30)]
    pub habilidad: String,
    pub encontrado_el: i64, // Timestamp de Unix
}

#[derive(Accounts)]
pub struct NuevaPc<'info> {
    #[account(mut)] 
    pub owner: Signer<'info>, 

    #[account(
        init, 
        payer = owner, 
        space = PcCaja::INIT_SPACE + 8, 
        seeds = [b"pc-caja", owner.key().as_ref()], 
        bump 
    )]
    pub pc_caja: Account<'info, PcCaja>, 

    pub system_program: Program<'info, System>, 
}

#[derive(Accounts)]
pub struct GestionarPokemon<'info> {
    pub owner: Signer<'info>, 
    #[account(mut)] 
    pub pc_caja: Account<'info, PcCaja>, 
}
