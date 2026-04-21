use anchor_lang::prelude::*;

declare_id!("AkabfcrZ6XsABTxkvbZtynKNndHphU9GUyXULNcWsXCZ");

#[program]
pub mod pc_pokemon {
    use super::*;

    // 1. CREATE: Inicializar una nueva caja (PDA por caja basada en índice)
    pub fn inicializar_pc(
        ctx: Context<NuevaPc>,
        nombre_entrenador: String,
        indice_caja: u8,
    ) -> Result<()> {
        require!(indice_caja < 32, Errores::LimiteCajasAlcanzado);

        let owner_id = ctx.accounts.owner.key();
        let equipo: Vec<Pokemon> = Vec::new();

        ctx.accounts.pc_caja.set_inner(PcCaja {
            owner: owner_id,
            nombre_entrenador,
            equipo,
            indice_caja,
        });
        Ok(())
    }

    // 2. CREATE (Insert): Depositar Pokémon en el vector
    pub fn depositar_pokemon(
        ctx: Context<GestionarPokemon>,
        especie: String,
        nivel: u8,
        pokedex_num: u16,
        sexo: String,
        tipos: Vec<String>,
        naturaleza: String,
        habilidad: String,
    ) -> Result<()> {
        require!(
            ctx.accounts.pc_caja.owner == ctx.accounts.owner.key(),
            Errores::NoEsTuPc
        );
        require!(
            (ctx.accounts.pc_caja.equipo.len() as u8) < 30,
            Errores::CajaLlena
        );

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

        ctx.accounts.pc_caja.equipo.push(nuevo_pokemon);
        Ok(())
    }

    // 3. READ: Ver los Pokémon guardados
    pub fn ver_pc(ctx: Context<GestionarPokemon>) -> Result<()> {
        require!(
            ctx.accounts.pc_caja.owner == ctx.accounts.owner.key(),
            Errores::NoEsTuPc
        );

        msg!(
            "Caja #{} de {}: {:#?}",
            ctx.accounts.pc_caja.indice_caja,
            ctx.accounts.pc_caja.nombre_entrenador,
            ctx.accounts.pc_caja.equipo
        );
        Ok(())
    }

    // 4. UPDATE: Alternar estado Shiny
    pub fn alternar_shiny(ctx: Context<GestionarPokemon>, especie: String) -> Result<()> {
        require!(
            ctx.accounts.pc_caja.owner == ctx.accounts.owner.key(),
            Errores::NoEsTuPc
        );

        let equipo = &mut ctx.accounts.pc_caja.equipo;
        for i in 0..equipo.len() {
            if equipo[i].especie == especie {
                let actual = equipo[i].es_shiny;
                equipo[i].es_shiny = !actual;
                return Ok(());
            }
        }
        Err(Errores::PokemonNoEncontrado.into())
    }

    // 5. DELETE: Liberar un Pokémon
    pub fn liberar_pokemon(ctx: Context<GestionarPokemon>, especie: String) -> Result<()> {
        require!(
            ctx.accounts.pc_caja.owner == ctx.accounts.owner.key(),
            Errores::NoEsTuPc
        );

        let equipo = &mut ctx.accounts.pc_caja.equipo;
        for i in 0..equipo.len() {
            if equipo[i].especie == especie {
                equipo.remove(i);
                return Ok(());
            }
        }
        Err(Errores::PokemonNoEncontrado.into())
    }
}

// ---- ERRORES ----
#[error_code]
pub enum Errores {
    #[msg("Acceso denegado: Esta PC pertenece a otro entrenador.")]
    NoEsTuPc,
    #[msg("El Pokémon especificado no se encuentra en esta caja.")]
    PokemonNoEncontrado,
    #[msg("No puedes crear más de 32 cajas.")]
    LimiteCajasAlcanzado,
    #[msg("La caja de Pokémon está llena (30 Pokémon máximo).")]
    CajaLlena,
}

// ---- ESTRUCTURAS Y ESPACIOS ----
#[account]
pub struct PcCaja {
    pub owner: Pubkey,
    pub nombre_entrenador: String,
    pub equipo: Vec<Pokemon>,
    pub indice_caja: u8, // Identificación distinta para cada caja
}

impl PcCaja {
    pub const MAX_NOMBRE: usize = 40;
    pub const MAX_EQUIPO: usize = 30;

    // Tamaños y espacios en bytes calculados manualmente
    pub const INIT_SPACE: usize = 32 // Pubkey owner
        + 4 + Self::MAX_NOMBRE * 4 // nombre_entrenador (string max 40 chars)
        + 4 + (Pokemon::SIZE * Self::MAX_EQUIPO) // vector equipo con 30 Pokémon
        + 1; // indice_caja u8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct Pokemon {
    pub especie: String,
    pub nivel: u8,
    pub es_shiny: bool,
    pub pokedex_num: u16,
    pub sexo: String,
    pub tipos: Vec<String>, 
    pub naturaleza: String,
    pub habilidad: String,
    pub encontrado_el: i64, // timestamp unix
}

impl Pokemon {
    // Tamaño calculado en bytes para espacio en cuenta (aprox)
    pub const SIZE: usize = 4 + 30 * 4  // especie String
        + 1  // nivel u8
        + 1  // es_shiny bool
        + 2  // pokedex_num u16
        + 4 + 15 * 4 // sexo String
        + 4 + 2 * 20 * 4 // tipos Vec<String> (2 tipos max 20 chars cada uno)
        + 4 + 20 * 4 // naturaleza String
        + 4 + 30 * 4 // habilidad String
        + 8; // encontrado_el i64
}

// ---- CONTEXTOS DE INSTRUCCIONES ----
#[derive(Accounts)]
#[instruction(indice_caja: u8)]
pub struct NuevaPc<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = PcCaja::INIT_SPACE + 8, // +8 por discrimnator anchor
        seeds = [b"pc-caja", owner.key().as_ref(), &[indice_caja]],
        bump,
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
