mod genes {
    use layout::Keyboard;

    struct Population {
        individuals: Vec<Individuals>,
        average_fitness: usize,
        best_fitness: usize,
        generation: usize,
    }

    struct Individual {
        chromosomes: Keyboard,
        fitness: usize,
    }

    pub fn mate(a: Individual, b: Individual) -> Individual {
        todo!();
    }
    impl Individual {
        fn mutate(&mut self) {}
        fn fitness(&mut self) {}
    }

    impl Population {
        fn prune(&mut self) {}
    }
}
