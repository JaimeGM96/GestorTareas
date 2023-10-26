use std::collections::HashMap as Map;
use chrono::NaiveTime;

type NumParada = i32;
type NumLinea = i32;

/**
 * Linea de autobús (en un único sentido).
 */
struct Linea {
	id: NumLinea,
	paradas: Vec<NumParada>,
	horarios: Map<NumParada, Vec<NaiveTime>>,
}

/**
 * Acción de subir/bajar de un autobús.
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

struct BuscadorRutas {
	lineas: Map<NumLinea, Linea>,
	paradas: Map<NumParada, Vec<NumLinea>>,
}

impl BuscadorRutas {
	pub fn new(lineas: Vec<Linea>) -> Lineas {

	}

	pub fn encuentra(hora_salida: NaiveTime, parada_origen: NumParada, parada_destino: NumParada) -> Option<Vec<Ruta>> {
		// lista de todas las posibles rutas desde parada_origen hasta parada_destino a partir de una hora en concreto
	}
}

fn consulta_mas_rapida(buscador: &BuscadorRutas, hora_salida: NaiveTime, parada_origen: NumParada, parada_destino: NumParada) -> Option<Ruta> {
	// min( ruta.back.hora - ruta.front.hora )
}