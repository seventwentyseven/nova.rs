use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Privileges: u32 {
        const Unrestricted = 1<<0;
        const Verified = 1<<1;
        const Frozen = 1<<2;
        const Szef = 1<<3;

        const WhitelistedStdVn = 1<<4;
        const WhitelistedTaikoVn = 1<<5;
        const WhitelistedCatchVn = 1<<6;
        const WhitelistedManiaVn = 1<<7;
        const WhitelistedStdRx = 1<<8;
        const WhitelistedTaikoRx = 1<<9;
        const WhitelistedCatchRx = 1<<10;
        const WhitelistedStdAp = 1<<11;

        const Supporter = 1<<12;
        const Alumni = 1<<13;
        const TourneyManager = 1<<14;

        const NominatorStd = 1<<15;
        const NominatorTaiko = 1<<16;
        const NominatorCatch = 1<<17;
        const NominatorMania = 1<<18;

        const QatStd = 1<<19;
        const QatTaiko = 1<<20;
        const QatCatch = 1<<21;
        const QatMania = 1<<22;

        const Moderator = 1<<23;
        const CommunityManager = 1<<24;
        const Administrator = 1<<25;
        const HeadAdmin = 1<<26;
        const Developer = 1<<27;
        const Owner = 1<<28;

        const Staff = Self::Moderator.bits()     | Self::CommunityManager.bits() |
                      Self::Administrator.bits() | Self::HeadAdmin.bits() |
                      Self::Developer.bits()     | Self::Owner.bits();

        const QATs = Self::QatStd.bits()   | Self::QatTaiko.bits() |
                     Self::QatCatch.bits() | Self::QatMania.bits();

        const Nominators =  Self::NominatorStd.bits()   | Self::NominatorTaiko.bits() |
                            Self::NominatorCatch.bits() | Self::NominatorMania.bits();

        const Whitelisted = Self::WhitelistedStdVn.bits()   | Self::WhitelistedTaikoVn.bits() |
                            Self::WhitelistedCatchVn.bits() | Self::WhitelistedManiaVn.bits() |
                            Self::WhitelistedStdRx.bits()   | Self::WhitelistedTaikoRx.bits() |
                            Self::WhitelistedCatchRx.bits() | Self::WhitelistedStdAp.bits();

        const Perks = Self::Supporter.bits() | Self::Alumni.bits() | Self::Staff.bits() |
                      Self::QATs.bits()      | Self::Nominators.bits();
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ClientPrivileges: u8 {
        const Player = 1 << 0;
        const Moderator = 1 << 1;
        const Supporter = 1 << 2;
        const Owner = 1 << 3;
        const Developer = 1 << 4;
        const Tournament = 1 << 5;  // NOTE: not used in communications with osu! client
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ClanPrivileges {
    Member = 1,
    Officer = 2,
    Owner = 3,
}
