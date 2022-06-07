extern crate near_sdk;
use near_sdk::{metadata, near_bindgen, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{self, Deserialize, Serialize};

use zksnark::groth16::fr::FrLocal;
use zksnark::setup_file::{SetupFile, CHECK, Fileish};
use zksnark::proof_file::{ProofFile};

#[derive(BorshDeserialize, BorshSerialize, Debug, Default, Serialize, Deserialize, Clone)]
#[near_bindgen]
struct SetupContract {
    setup_file: SetupFile
}

#[near_bindgen]
impl SetupContract {

    pub fn default () -> Self {
        Self {
            setup_file: SetupFile::default()
        }
    }

    pub fn verify(&self, assignments: Vec<FrLocal>, proof: ProofFile) -> bool {
        return self.setup_file.verify(assignments, proof)
    }

    pub fn from_zk(&mut self, code: &String) -> Self {
        self.setup_file = SetupFile::from_zk(code);
        self.clone()      
    }

    pub fn from_setup(&mut self, setup_file: SetupFile) -> Self {
        self.setup_file = setup_file;
        self.clone()
    }

    pub fn from_hex_string(&mut self, hex: &String) -> Self {
        self.setup_file = SetupFile::from_hex_string(hex.to_string());
        self.clone()
    }

    pub fn to_hex_string(&self) -> String {
        self.setup_file.to_hex_string()
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    const TEST_HEX: &str = "ea1dadab7200000028696e206120622063290a286f75742078290a2876657269667920622078290a0a2870726f6772616d0a20202020283d2074656d700a2020202020202020282a2061206229290a20202020283d20780a2020202020202020282a203120282b20282a20342074656d702920632036292929290600000002000000060000a077c14b9767a358dab27137f12e12080947a2e151fac02947b1d65922fbffff4f1c3496ac29cd609f9576fc362e4679786fa36e662fdf079ac1770a0e01000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000002000000f6ffff9f38682c59539ac13e2bedf86d5c8cf2f0de46ddcc5ebe0f3483ef141c060000a077c14b9767a358dab27137f12e12080947a2e151fac02947b1d659220100000000000000000000000000000000000000000000000000000000000000000000000600000002000000200000007eb23e7c28122e370f097d06a50b2b30d0b6080a370534265cce890ce1ffffef1543a3c7685e8b4239dfb621b84c5651e68e47aef29afdba1680da2302000000f6ffff9f38682c59539ac13e2bedf86d5c8cf2f0de46ddcc5ebe0f3483ef141c060000a077c14b9767a358dab27137f12e12080947a2e151fac02947b1d6592201000000000000000000000000000000000000000000000000000000000000000000000002000000160000a0b61a6bd57bacef753af6757401981d21affde5d695c3435adfbd9e28ebffff4fddda766e15c4c9030ef2bdb35bc0636007486ae193dced869390c50701000000000000000000000000000000000000000000000000000000000000000000000002000000060000a077c14b9767a358dab27137f12e12080947a2e151fac02947b1d65922fbffff4f1c3496ac29cd609f9576fc362e4679786fa36e662fdf079ac1770a0e0600000001000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000002000000060000a077c14b9767a358dab27137f12e12080947a2e151fac02947b1d65922fbffff4f1c3496ac29cd609f9576fc362e4679786fa36e662fdf079ac1770a0e02000000f6ffff9f38682c59539ac13e2bedf86d5c8cf2f0de46ddcc5ebe0f3483ef141c060000a077c14b9767a358dab27137f12e12080947a2e151fac02947b1d6592201000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000003000000f6ffff9f38682c59539ac13e2bedf86d5c8cf2f0de46ddcc5ebe0f3483ef141c100000003f591f3e1409979b87843e83d2851518685b04859b021a132ee74406fbffff4f1c3496ac29cd609f9576fc362e4679786fa36e662fdf079ac1770a0e020000000000000002000000000000009d28decfbe0adc612ce6b1f38769ad9be5266fcd4d614f966437814ca08b821f3d679792659bf927bd8bb15384ac66de7c872baff5958f72bf2e5e447d567f23cdeee9be7ad55b6ea0cdc956122ff9f2c08ed25ac1378cb1f8445c93a0ee8e158b4919c5a6cfea9c9d102e876f8a3515a7fedafeb8a6ecf65da1f737155be822923e0f0b352efa346977cbf2ce148f3157e02fbbb25a785678d6f10215a27f2bd14907c6f82e9e02ddb30c49eb995631ec5ce3a8b68be9c5eaa243b1ada7b006799b200e073bcfc99ad4cd1f68a74b7c43728a874223b9b639d02648d5a589091e3c6f3fde2ce2cd4d00923fc1ac5ebb434af7afa57442fb6d99f670206ad80ff1a2a7183ff785088e40cfacfb66f745b5b2c8279e1b2a1d4dbc0d97884ddd1202000000c2ea4fdeb51cd12c47fd59467bcad41328085f585240da993266bba9ce796205002e8c3b557874922e480b48cc1e48725fad43980f1879d9968e260a1c9049164350e55d4ce846eb38d88dc708661139b38e26ceaa3d48ff3372fb1608d28d0be43f66849244ba601d46616fffb94712cd2d400c2a2037ff22b6e4d4ded2c32c4914aef3653fe3a115949775d61be2db9fe144242ec0dde669c943c78657012f2d4d1b803c4cf2c4e5da24882b5095322f7273c9efcd6a5f85c61750d0ff810c030000006ddc3e433b8e23efffe4f6f5d20f455ebfb2560ffbb95d630a2a5b05a1f4252f1143721dd0c9c3bbef60b8ffbed405eb9bb1dffbf06e69fc27dae08710e2ff1a9e14488a98c27838b8d66bf29b6a2ed8be1b2b8450cb6680903d4b0b0d954c2b9b02c91161c673cc2f04dad454847dad8b2fe5d648edd59d9ef7419e73390726ca16989a289bf43aaca284561d2a63cc99e596b7b1e8353036ff9d731744990dff33b2d5d2a6a6140b3373e14b8ba46e04d1459a0999342d39d2c157257b480605a19a689c5c558f70ab4ccc0d8a7de38bda7320c98d34be2feee4956e6a0319867bede45baa5b1ddb2737956b1b06613f1f5de593f972c1d040833554d62f061d8ba51dac94c7964e848c6ed7af9559041b12fcca309c7beacf16537697d71a030000001e1229acf7ebe70de7cfdb5adfd71ed2206dc1442aa749a7ccd0bb53891e0215705d05b47ee89a2e151b22b3060219d91180c7ae9b68402b362b5701075f592a2edd22965bcc691546dc8ef91940b6224e66c7475b33604d90e9385ee6890321dd96c707d79fc3da2ad784c4da175134ab85923dc8c01ee29da0ee6a0d036d261275e11410bc09200012e4d622974600bea2c6517a863caac56d58b38ed983138e1ab17f10a92f800a61ab45f9c23bfb5c0d989a30b29357fa9edac5f27eab115a114b023997dc9bee330480084f95b9e1ab3bc6f2afe86dbeb0340dc1238007157079cba35e89a4b652653fee51a903a6e46aa559a57193589deaf8505a6e16e10bcbe18852f04fc79c267ef13137b4ea218763a2d4e964f2514b3163e4d91a010000006564597bb641958a211a4df99e035e633ff596287e1982b697bbc81e36ce802c47004895d06e8816b6797a9a17dad7c9fef33d430a76434d395457c03d37db026ba5fdc6226706f5ad8de38388207106ea09d73b73026d227dd7bfa75178392701f28058c2deb6cc569bf146fcc7c74b1c4168bac2194887d93488f8e504a604dc16592c3719d5c36bdf1b50722b9656533a2cb13bb362d83ff75f9add36051964d87560720b56d689f236de825915ffbb61edae39cf2ec9cb18bff1baff2b287e09a284c8223f4ea91893e04f3ffbbd16275ca27d991655f7943bd45388631a662ee00b257ad1c7b8564e3354b1f72237fba80a98f8843125792f9b1f33972681161ea03c0a2ac616714a48dede473ba1da67116f7a015b462006955f6bd61a63fcd9583ec777e4f925c565663e9ef20bfc55422ac403331d01728b61bd021a18632907bb398154b331fed2340fdd18c041df96a0cebf0d8a026b4cf32a2c090679573da2f2d3b6d4eb0e08c5dce07108c5443d25c4e5e4e0e8e648cfac520c0a7df9f1a67e895d0580ff3456448ea6543875dd6caa8355650d2785d568ab2402d9e37e9df55b2894b9a8617a640c2bc36424b204667305909708a1fddad1103ac74b9d434cf9ca8d5c1b9d09141b8b2a177a3b99b0c67777ec84003ab89611e7b5f886155f11e57788a43c52a5e4b562e99e570d8d515a216de261ebcf362ed64f7586e6b36dd9ea5e3b926834f36be7f8157d2d246b49f854618b6c07e8298fc204db2774b30f0c709fb6839148936645b3445bae203acf510b241c8eff0a002868492cc6caace80b20ed63a0261188932c71eb16bd9e4bd9c16f8b7a231782ed083aa9ea66e8d3a257d8a126d5342cc652634ba5d474ec5c995f678db311b9aa4c253c3a5cd8492b9efdbe8a1d501b23b245750f359b97cb03b12acd0f23020000005d4567e7577477e27ff4a2cd8de9facb564563add1dd97a0f5da2c81d67f650e10e9ed90696c68b62dce18933231248f542aef40602c910b3f0262de2492d02a8601289378f2595647441bb4ac09734a17b8fb049dc735c7afb3e3c585bc5f18348e3372fea16fa844daf1114958493b7f2501436e4961f4f64415e40e650314dd013bc0e0ef4019bf3f08ab8d750771ce7b2f2938f7c7ef38757b26733dbb0691e074f8f664347d5c738bc6b0642c3a6230cc9190e982401c4952e4b58b1a16301b5a2d2518cb1bb6cfa089aeb4f4c6577009eeb05b3c1ce695e77d3e8dc0014f2b5cadd66e2bb1bef45b113a5619e93e5e5e21b768edf1f5cec57fd2bb891efd1ac437a932ac00b2272fbb9f818cf9dd795a935e8da86836c783f58228ae1f27e27d8d3d02bab307e6266d6577a28b8a0eed939e883fb9a363f83c46f6d10832ee841905ba0618b31e8d0cf8d407031df999dd8ae6fdf15d4923415280ad2610638354d9035f888811ddccb23ae5f44eb28d882395cc2814121772a61aa10c";

    const TEST_ZK: &str = "
        (in a b c)
        (out x)
        (verify b x)

        (program
            (= temp
                (* a b))
            (= x
                (* 1 (+ (* 4 temp) c 6))))
    ";

    fn input_assignments() -> [FrLocal; 3] {
        return [
            FrLocal::from(3), // a
            FrLocal::from(2), // b
            FrLocal::from(4) // c
        ];
    } 

    fn output_assignments() -> Vec<FrLocal> {
        return vec! [
            FrLocal::from(2),
            FrLocal::from(34)
        ];
    } 

    #[test]
    fn from_zk_test() {
        let context = get_context(false);
        testing_env!(context);
        let contract = SetupContract::default().from_zk(&String::from(TEST_ZK));
        assert_eq!(
            contract.setup_file.check,
            CHECK
        );
    }

    #[test]
    fn from_hex_test() {
        let context = get_context(false);
        testing_env!(context);
        let contract = SetupContract::default().from_hex_string(&String::from(TEST_HEX));
        assert_eq!(
            contract.setup_file.check,
            CHECK
        );
    }

    #[test]
    fn setup_verify_test() {
        let context = get_context(false);
        testing_env!(context);
        let setup_file = SetupFile::from_zk(TEST_ZK);
        let setup_contract = SetupContract { setup_file: setup_file.clone()};
        
        let proof_file =  ProofFile::from_setup(&input_assignments(), setup_file);

        assert!(setup_contract.verify(output_assignments(), proof_file))
    }

    use workspaces::prelude::*;

    #[tokio::test]
    async fn test_cross_contract() -> anyhow::Result<()> {
        let worker = workspaces::sandbox().await?;
        let contract = worker.dev_deploy(&std::fs::read(format!("../res/setup.wasm"))?).await?;

        let res = contract
            .call(&worker, "from_zk")
            .args_json(TEST_ZK)?
            .gas(300_000_000_000_000)
            .transact()
            .await?;
        assert!(res.is_success());

        Ok(())
    }

}
