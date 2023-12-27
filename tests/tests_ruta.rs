#[cfg(test)]
mod tests {
    use buscador_rutas::*;
    use chrono::NaiveTime;
    use std::collections::HashMap as Map; 

    #[test]
    fn test_a_linea_can_be_created() {
        let mut horarios = Map::new();
        horarios.insert(102, vec![NaiveTime::from_hms(7, 0, 0)]);

        let linea = Linea {
            id: 1,
            paradas: vec![101, 102, 103],
            horarios: horarios,
        };

        assert_eq!(linea.id, 1);
        assert_eq!(linea.paradas.len(), 3);
        assert_eq!(linea.horarios.len(), 1);
    }

    #[test]
    fn test_a_transbordo_can_be_created() {
        let transbordo = Transbordo {
            linea: 1,
            parada: 101,
            hora: NaiveTime::from_hms(7, 0, 0),
        };

        assert_eq!(transbordo.linea, 1);
        assert_eq!(transbordo.parada, 101);
        assert_eq!(transbordo.hora, NaiveTime::from_hms(7, 0, 0));
    }

    #[test]
    fn test_a_buscador_rutas_can_be_created() {
        let mut horarios = Map::new();
        horarios.insert(102, vec![NaiveTime::from_hms(7, 0, 0)]);

        let linea = Linea {
            id: 1,
            paradas: vec![101, 102, 103],
            horarios: horarios,
        };

        let mut paradas = Map::new();
        paradas.insert(101, vec![1]);
        paradas.insert(102, vec![1, 2]);

        let buscador_rutas = BuscadorRutas::new(vec![linea], paradas).unwrap();

        assert_eq!(buscador_rutas.lineas.len(), 1);
        assert_eq!(buscador_rutas.paradas.len(), 2);
    }

    #[test]
    fn test_a_buscador_rutas_cannot_be_created_without_paradas() {
        let horarios = Map::new();

        let linea = Linea {
            id: 1,
            paradas: vec![101, 102, 103],
            horarios: horarios,
        };

        let paradas = Map::new();

        let buscador_rutas = BuscadorRutas::new(vec![linea], paradas);

        assert_eq!(buscador_rutas.err(), Some("No se puede crear un buscador de rutas sin paradas"));
    }

    #[test]
    fn test_a_buscador_rutas_cannot_be_created_without_lineas() {
        let horarios = Map::new();

        let linea = Linea {
            id: 1,
            paradas: vec![101, 102, 103],
            horarios: horarios,
        };

        let mut paradas = Map::new();
        paradas.insert(101, vec![1]);
        paradas.insert(102, vec![1, 2]);

        let buscador_rutas = BuscadorRutas::new(vec![], paradas);

        assert_eq!(buscador_rutas.err(), Some("No se puede crear un buscador de rutas sin lineas"));
    }

    #[test]
    pub fn test_given_a_buscador_rutas_can_get_rutas() {
        let mut horarios = Map::new();
        horarios.insert(101, vec![NaiveTime::from_hms(7, 0, 0)]);
        horarios.insert(102, vec![NaiveTime::from_hms(7, 10, 0)]);
        horarios.insert(103, vec![NaiveTime::from_hms(7, 20, 0)]);


        let linea = Linea {
            id: 1,
            paradas: vec![101, 102, 103],
            horarios: horarios,
        };

        let mut paradas = Map::new();
        paradas.insert(101, vec![1]);
        paradas.insert(102, vec![1, 2]);

        let buscador_rutas = BuscadorRutas::new(vec![linea], paradas);

        let rutas = buscador_rutas.unwrap().encuentra(NaiveTime::from_hms(7, 0, 0), 101, 103);

        assert_eq!(rutas.unwrap().len(), 1);
    }
}