#![allow(dead_code)]
#![allow(unused_variables)]	// quitar cuando se implemente

use std::collections::HashMap as Map;
use chrono::NaiveTime;

type NumParada = i32;
type NumLinea = i32;

/**
 * Linea de autobús (en un único sentido).
 * 
 * Entidad, al estar unequivocamente identificado.
 */
pub struct Linea {
	pub id: NumLinea,
	pub paradas: Vec<NumParada>,
	pub horarios: Map<NumParada, Vec<NaiveTime>>,
}

/**
 * Acción de subir/bajar de un autobús.
 * 
 * Objeto valor, al ser inmutable y no estar
 * identificado unequivocamente.
 */
pub struct Transbordo {
	pub linea: NumLinea,
	pub parada: NumParada,
	pub hora: NaiveTime,
}

/**
 * Una ruta es una consecución de subidas/bajadas
 * de autobuses.
 */
type Ruta = Vec<Transbordo>;

/**
 * Información necesaria indexada para poder
 * buscar rutas entre dos paradas.
 * 
 * Objeto valor, al ser inmutable y no estar
 * identificado unequivocamente.
 */
pub struct BuscadorRutas {
	pub lineas: Map<NumLinea, Linea>,
	pub paradas: Map<NumParada, Vec<NumLinea>>,
}

impl BuscadorRutas {
	pub fn new(lineas: Vec<Linea>, paradas: Map<NumParada, Vec<NumLinea>>) -> Self {
		let mut buscador = Self {
			lineas: Map::new(),
			paradas,
		};

		for linea in lineas {
			buscador.lineas.insert(linea.id, linea);
		}

		buscador
	}

	pub fn encuentra(hora_salida: NaiveTime, parada_origen: NumParada, parada_destino: NumParada) -> Option<Vec<Ruta>> {
		// lista de todas las posibles rutas desde parada_origen hasta parada_destino a partir de una hora en concreto
		//Para una linea que cubre la parada origen y la parada destino
		None
	}
}

/**
 * Devuelve la ruta más rápida entre dos paradas
 * a partir de una hora en concreto.
 * 
 * Servicio, al no estar asociado particularmente
 * a un objeto del dominio.
 */
fn consulta_mas_rapida(buscador: &BuscadorRutas, hora_salida: NaiveTime, parada_origen: NumParada, parada_destino: NumParada) -> Option<Ruta> {
	// min( ruta.back.hora - ruta.front.hora )
	None
}
