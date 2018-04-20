macro_rules! dot_dense {
    ($scalar:ident => ($lhs:expr, $rhs:expr)) => {
        $lhs.into_iter()
          .zip($rhs.into_iter())
          .fold($scalar::zero(),
          |sum, ((lhs_i, lhs), (rhs_i, rhs))| {
              debug_assert_eq!(lhs_i, rhs_i);
              sum + (lhs * rhs)
          })
    }
}

macro_rules! squared_distance_dense {
    ($scalar:ident => ($lhs:expr, $rhs:expr)) => {
        $lhs.into_iter()
            .zip($rhs.into_iter())
            .fold($scalar::zero(),
                  |sum, ((lhs_i, lhs), (rhs_i, rhs))| {
                      debug_assert_eq!(lhs_i, rhs_i);
                      let delta = lhs - rhs;
                      sum + (delta * delta)
                  })
    }
}
