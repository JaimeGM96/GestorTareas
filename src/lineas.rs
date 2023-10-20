use chrono::NaiveTime;

type NumParada = i32;

struct Linea {
	id: i32,
	paradas: Vec<NumParada>,
	horarios: Map<NumParada, Vec<NaiveTime>>,
}
