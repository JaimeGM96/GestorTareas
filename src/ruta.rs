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
struct Linea {
	id: NumLinea,
	paradas: Vec<NumParada>,
	horarios: Map<NumParada, Vec<NaiveTime>>,
}

/**
 * Acción de subir/bajar de un autobús.
 * 
 * Objeto valor, al ser inmutable y no estar
 * identificado unequivocamente.
 */
struct Transbordo {
	linea: NumLinea,
	parada: NumParada,
	hora: NaiveTime,
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
struct BuscadorRutas {
	lineas: Map<NumLinea, Linea>,
	paradas: Map<NumParada, Vec<NumLinea>>,
}

impl BuscadorRutas {
	pub fn new(lineas: Vec<Linea>) -> Self {
		Self {
			lineas: Map::new(),
			paradas: Map::new(),
		}
	}

	pub fn encuentra(hora_salida: NaiveTime, parada_origen: NumParada, parada_destino: NumParada) -> Option<Vec<Ruta>> {
		// lista de todas las posibles rutas desde parada_origen hasta parada_destino a partir de una hora en concreto
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
