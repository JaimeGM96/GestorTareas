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

        let buscador_rutas = BuscadorRutas::new(vec![linea], paradas);

        assert_eq!(buscador_rutas.lineas.len(), 1);
        assert_eq!(buscador_rutas.paradas.len(), 2);
    }
}