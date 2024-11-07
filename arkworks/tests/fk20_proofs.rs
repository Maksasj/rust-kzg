#[cfg(test)]
mod tests {
    use kzg_bench::tests::fk20_proofs::*;

    use rust_kzg_arkworks::eip_7594::ArkBackend;
    use rust_kzg_arkworks::fk20_proofs::{KzgFK20MultiSettings, KzgFK20SingleSettings};
    use rust_kzg_arkworks::kzg_proofs::generate_trusted_setup;

    #[ignore = "KZG settings loading doesn't support trusted setup sizes other than FIELD_ELEMENTS_PER_BLOB (4096 points)"]
    #[test]
    fn test_fk_single() {
        fk_single::<ArkBackend, KzgFK20SingleSettings>(&generate_trusted_setup);
    }

    #[ignore = "KZG settings loading doesn't support trusted setup sizes other than FIELD_ELEMENTS_PER_BLOB (4096 points)"]
    #[test]
    fn test_fk_single_strided() {
        fk_single_strided::<ArkBackend, KzgFK20SingleSettings>(&generate_trusted_setup);
    }

    #[ignore = "KZG settings loading doesn't support trusted setup sizes other than FIELD_ELEMENTS_PER_BLOB (4096 points)"]
    #[test]
    fn test_fk_multi_settings() {
        fk_multi_settings::<ArkBackend, KzgFK20MultiSettings>(&generate_trusted_setup);
    }

    #[ignore = "KZG settings loading doesn't support trusted setup sizes other than FIELD_ELEMENTS_PER_BLOB (4096 points)"]
    #[test]
    fn test_fk_multi_chunk_len_1_512() {
        fk_multi_chunk_len_1_512::<ArkBackend, KzgFK20MultiSettings>(&generate_trusted_setup);
    }

    #[ignore = "KZG settings loading doesn't support trusted setup sizes other than FIELD_ELEMENTS_PER_BLOB (4096 points)"]
    #[test]
    fn test_fk_multi_chunk_len_16_512() {
        fk_multi_chunk_len_16_512::<ArkBackend, KzgFK20MultiSettings>(&generate_trusted_setup);
    }

    #[ignore = "KZG settings loading doesn't support trusted setup sizes other than FIELD_ELEMENTS_PER_BLOB (4096 points)"]
    #[test]
    fn test_fk_multi_chunk_len_16_16() {
        fk_multi_chunk_len_16_16::<ArkBackend, KzgFK20MultiSettings>(&generate_trusted_setup);
    }
}
