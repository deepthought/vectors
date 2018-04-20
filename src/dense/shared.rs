macro_rules! dot_dense {
    ($scalar:ident => ($lhs:expr, $rhs:expr)) => ({
        let lhs_iter = $lhs.into_iter();
        let rhs_iter = $rhs.into_iter();
        debug_assert_eq!(lhs_iter.len(), rhs_iter.len());
        lhs_iter.zip(rhs_iter).fold($scalar::zero(), |sum, ((_, lhs), (_, rhs))| {
            sum + (lhs * rhs)
        })
    })
}

macro_rules! squared_distance_dense {
    ($scalar:ident => ($lhs:expr, $rhs:expr)) => ({
        let lhs_iter = $lhs.into_iter();
        let rhs_iter = $rhs.into_iter();
        debug_assert_eq!(lhs_iter.len(), rhs_iter.len());
        lhs_iter.zip(rhs_iter).fold($scalar::zero(), |sum, ((_, lhs), (_, rhs))| {
            let delta = lhs - rhs;
            sum + (delta * delta)
        })
    })
}
