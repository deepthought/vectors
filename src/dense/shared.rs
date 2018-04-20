macro_rules! dot {
    ($scalar:ident => ($lhs:expr, $rhs:expr)) => {
        $lhs.iter()
          .zip($rhs.into_iter())
          .fold($scalar::zero(),
                |sum, ((_, lhs), (_, rhs))| sum + (lhs * rhs))
    }
}

macro_rules! squared_distance_generic {
    ($scalar:ident => ($lhs:expr, $rhs:expr)) => {
        $lhs.iter()
            .zip($rhs.into_iter())
            .fold($scalar::zero(),
                  |sum, ((_, lhs), (_, rhs))| {
                      // We might be dealing with an unsigned scalar type.
                      // As such just doing `lhs - rhs` might lead to underfows:
                      let delta = match lhs.partial_cmp(&rhs) {
                          Some(Ordering::Less) => rhs - lhs,
                          Some(Ordering::Equal) => $scalar::zero(),
                          Some(Ordering::Greater) => lhs - rhs,
                          None => $scalar::zero(),
                      };
                      sum + (delta * delta)
                  })
    }
}

#[cfg(feature = "use-specialization")]
macro_rules! squared_distance_signed {
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
