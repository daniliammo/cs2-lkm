use driver::Driver;
use cs2_dumper::libclient_so::cs2_dumper::schemas;

pub enum WeaponClass {
    Invalid,
    Knife,
    Grenade,
    Pistol,
    Sniper,
    Rifle,
    Shotgun,
}

pub fn get_weapon_class(driver: &Driver, pawn: usize) -> WeaponClass {
    let weapon: usize = driver.read_mem(pawn + schemas::libclient_so::C_CSPlayerPawnBase::m_pClippingWeapon);

    if weapon == 0 {
        return WeaponClass::Invalid;
    }

    let weapon_index: i16 = driver.read_mem(
        weapon + schemas::libclient_so::C_EconEntity::m_AttributeManager 
        + schemas::libclient_so::C_AttributeContainer::m_Item 
        + schemas::libclient_so::C_EconItemView::m_iItemDefinitionIndex,
    );

    let knife_weapons = [42, 59];
    if knife_weapons.contains(&weapon_index) {
        return WeaponClass::Knife;
    }

    let grenade_weapons = [44, 43, 45, 47, 46, 48, 49];
    if grenade_weapons.contains(&weapon_index) {
        return WeaponClass::Grenade;
    }

    let pistol_weapons = [40, 32, 61, 1, 36, 2, 3, 4, 30];
    if pistol_weapons.contains(&weapon_index) {
        return WeaponClass::Pistol;
    }

    let sniper_weapons = [9, 38, 11];
    if sniper_weapons.contains(&weapon_index) {
        return WeaponClass::Sniper;
    }

    let shotgun_weapons = [35, 29, 27, 25];
    if shotgun_weapons.contains(&weapon_index) {
        return WeaponClass::Shotgun;
    }

    WeaponClass::Rifle
}