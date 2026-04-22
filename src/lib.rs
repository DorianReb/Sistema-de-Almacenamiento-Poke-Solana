use anchor_lang::prelude::*;

declare_id!("AkabfcrZ6XsABTxkvbZtynKNndHphU9GUyXULNcWsXCZ");

#[program]
pub mod pc_pokemon {
    use super::*;

    // ==========================================
    // 1. CREATE: Inicializar una caja específica (1 al 32)
    // ==========================================
    pub fn inicializar_pc(
        ctx: Context<NuevaPc>,
        numero_caja: u8, 
        nombre_entrenador: String,
    ) -> Result<()> {
        let pc_caja = &mut ctx.accounts.pc_caja;
        
        pc_caja.owner = ctx.accounts.owner.key();
        pc_caja.nombre_entrenador = nombre_entrenador;
        pc_caja.contador_ids = 0; 
        pc_caja.equipo = Vec::new();

        msg!("Caja #{} de {} inicializada con éxito.", numero_caja, pc_caja.nombre_entrenador);
        Ok(())
    }

    // ==========================================
    // 2. INSERT: Depositar Pokémon en la caja seleccionada
    // ==========================================
    pub fn depositar_pokemon(
        ctx: Context<GestionarPokemon>,
        numero_caja: u8,       
        especie: String,
        nivel: u8,
        pokedex_num: u16,
        sexo: Sexo,
        tipos: String, 
        naturaleza: Naturaleza,
        habilidad: String,
        ot_nombre: String,
    ) -> Result<()> {
        let pc_caja = &mut ctx.accounts.pc_caja;

        require!(pc_caja.owner == ctx.accounts.owner.key(), Errores::NoEsTuPc);
        require!((pc_caja.equipo.len() as u8) < 30, Errores::CajaLlena);

        let clock = Clock::get()?;
        
        pc_caja.contador_ids += 1;
        let id_unico = pc_caja.contador_ids;
        
        let nuevo_pokemon = Pokemon {
            id: id_unico,
            especie,
            nivel,
            es_shiny: false,
            pokedex_num,
            sexo,
            tipos,
            naturaleza,
            habilidad,
            encontrado_el: clock.unix_timestamp,
            ot_pubkey: ctx.accounts.owner.key(),
            ot_nombre,
        };

        msg!("Caja #{}: ¡{} (ID: {}) guardado!", numero_caja, nuevo_pokemon.especie, nuevo_pokemon.id);
        
        pc_caja.equipo.push(nuevo_pokemon);
        Ok(())
    }

    // ==========================================
    // 3. READ: Ver SOLO la caja seleccionada
    // ==========================================
    pub fn ver_pc(ctx: Context<GestionarPokemon>, numero_caja: u8) -> Result<()> {
        let pc_caja = &ctx.accounts.pc_caja;
        
        msg!("========================================");
        msg!("CAJA NÚMERO: #{}", numero_caja); 
        msg!("ENTRENADOR: {}", pc_caja.nombre_entrenador);
        msg!("POKÉMON EN ESTA CAJA: {}/30", pc_caja.equipo.len());
        msg!("TOTAL REGISTRADOS HISTÓRICOS: {}", pc_caja.contador_ids);
        msg!("========================================");
        
        // Si está vacía, avisamos
        if pc_caja.equipo.is_empty() {
            msg!("Esta caja está vacía actualmente.");
        } else {
            for pkmn in pc_caja.equipo.iter() {
                let texto_shiny = if pkmn.es_shiny { "Sí ✨" } else { "No" };

                msg!("ID: {} | {} (Pokedex: #{:04})", pkmn.id, pkmn.especie, pkmn.pokedex_num);
                msg!("Nvl: {} | Shiny: {} | Sexo: {:?}", pkmn.nivel, texto_shiny, pkmn.sexo);
                msg!("Naturaleza: {:?} | Habilidad: {}", pkmn.naturaleza, pkmn.habilidad);
                msg!("Tipos: {}", pkmn.tipos);
                msg!("OT: {} [{}]", pkmn.ot_nombre, pkmn.ot_pubkey.to_string());
                msg!("----------------------------------------");
            }
        }
        
        Ok(())
    }

    // ==========================================
    // 4. UPDATE: Alternar estado Shiny
    // ==========================================
    pub fn alternar_shiny(ctx: Context<GestionarPokemon>, numero_caja: u8, id_pokemon: u32) -> Result<()> {
        let pc_caja = &mut ctx.accounts.pc_caja;
        require!(pc_caja.owner == ctx.accounts.owner.key(), Errores::NoEsTuPc);

        let equipo = &mut pc_caja.equipo;
        for pkmn in equipo.iter_mut() {
            if pkmn.id == id_pokemon {
                pkmn.es_shiny = !pkmn.es_shiny;
                msg!("Caja #{}: Estado shiny de {} actualizado a: {}", numero_caja, pkmn.especie, pkmn.es_shiny);
                return Ok(());
            }
        }
        Err(Errores::PokemonNoEncontrado.into())
    }

    // ==========================================
    // 5. DELETE: Liberar un Pokémon
    // ==========================================
    pub fn liberar_pokemon(ctx: Context<GestionarPokemon>, numero_caja: u8, id_pokemon: u32) -> Result<()> {
        let pc_caja = &mut ctx.accounts.pc_caja;
        require!(pc_caja.owner == ctx.accounts.owner.key(), Errores::NoEsTuPc);

        let equipo = &mut pc_caja.equipo;
        let index = equipo.iter().position(|x| x.id == id_pokemon);

        if let Some(i) = index {
            let pkmn_liberado = equipo.remove(i);
            msg!("Caja #{}: Has liberado a {} (ID: {}). ¡Adiós, amigo!", numero_caja, pkmn_liberado.especie, pkmn_liberado.id);
            Ok(())
        } else {
            Err(Errores::PokemonNoEncontrado.into())
        }
    }
}

// ==========================================
// ENUMS
// ==========================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub enum Sexo {
    Macho,
    Hembra,
    SinGenero,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub enum Naturaleza {
    Activa, Hurana, Afable, Ingenua, Agitada, Mansa, Alegre, Miedosa, Alocada, 
    Modesta, Amable, Osada, Audaz, Picara, Cauta, Placida, Docil, Rara, Firme, 
    Serena, Floja, Seria, Fuerte, Timida, Grosera
}

// ==========================================
// ESTRUCTURAS DE DATOS 
// ==========================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Pokemon {
    pub id: u32,
    #[max_len(30)]
    pub especie: String,
    pub nivel: u8,
    pub es_shiny: bool,
    pub pokedex_num: u16,
    
    pub sexo: Sexo,
    pub naturaleza: Naturaleza,
    
    #[max_len(25)] 
    pub tipos: String,
    #[max_len(30)]
    pub habilidad: String,
    pub encontrado_el: i64,
    
    pub ot_pubkey: Pubkey,
    #[max_len(40)]
    pub ot_nombre: String,
}

#[account]
#[derive(InitSpace)]
pub struct PcCaja {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_entrenador: String,
    pub contador_ids: u32,
    #[max_len(30)] 
    pub equipo: Vec<Pokemon>,
}

// ==========================================
// CONTEXTOS DE INSTRUCCIONES
// ==========================================

#[derive(Accounts)]
#[instruction(numero_caja: u8)] 
pub struct NuevaPc<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + PcCaja::INIT_SPACE,
        seeds = [b"pc-caja", owner.key().as_ref(), &[numero_caja]],
        bump,
        constraint = numero_caja >= 1 && numero_caja <= 32 @ Errores::NumeroCajaInvalido
    )]
    pub pc_caja: Account<'info, PcCaja>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(numero_caja: u8)] // 👈 Añadido: Para forzar la lectura del argumento
pub struct GestionarPokemon<'info> {
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"pc-caja", owner.key().as_ref(), &[numero_caja]],
        bump,
        has_one = owner @ Errores::NoEsTuPc
    )]
    pub pc_caja: Account<'info, PcCaja>, 
}

// ==========================================
// ERRORES PERSONALIZADOS
// ==========================================

#[error_code]
pub enum Errores {
    #[msg("Acceso denegado: Esta PC pertenece a otro entrenador.")]
    NoEsTuPc,
    #[msg("El Pokémon especificado no se encuentra en esta caja.")]
    PokemonNoEncontrado,
    #[msg("La caja de Pokémon está llena (30 Pokémon máximo).")]
    CajaLlena,
    #[msg("El número de caja debe estar entre 1 y 32.")]
    NumeroCajaInvalido,
}
