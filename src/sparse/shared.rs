macro_rules! dot_sparse {
    ($scalar:ident => ($lhs:expr, $rhs:expr)) => {
        $lhs.into_iter()
            .inner_join_map($rhs.into_iter())
            .fold($scalar::zero(),
                  |sum, (_, (lhs, rhs))| sum + (lhs * rhs))
    }
}

macro_rules! squared_distance_sparse {
    ($scalar:ident => ($lhs:expr, $rhs:expr)) => {
        $lhs.into_iter()
            .inner_join_map($rhs.into_iter())
            .fold($scalar::zero(),
                  |sum, (_, (lhs, rhs))| {
                      let delta = lhs - rhs;
                      sum + (delta * delta)
                  })
    }
}
