macro_rules! dot_sparse {
    ($scalar:ident => ($lhs:expr, $rhs:expr)) => ({
        let lhs_iter = $lhs.into_iter();
        let rhs_iter = $rhs.into_iter();
        lhs_iter.inner_join_map(rhs_iter).fold($scalar::zero(), |sum, (_, (lhs, rhs))| {
            sum + (lhs * rhs)
        })
    })
}

macro_rules! squared_distance_sparse {
    ($scalar:ident => ($lhs:expr, $rhs:expr)) => ({
        let lhs_iter = $lhs.into_iter();
        let rhs_iter = $rhs.into_iter();
        lhs_iter.inner_join_map(rhs_iter).fold($scalar::zero(), |sum, (_, (lhs, rhs))| {
            let delta = lhs - rhs;
            sum + (delta * delta)
        })
    })
}
