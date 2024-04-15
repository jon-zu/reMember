use shroom_meta::id::{ItemId, NpcId};
use shroom_pkt::{with_opcode, ShroomList16, ShroomOption8, ShroomPacket, ShroomPacketEnum};

use crate::{recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes};

#[derive(ShroomPacket, Debug)]
pub struct ShopItem {
    pub item_id: ItemId,
    pub price: u32,
    pub discount_rate: u8,
    pub token_item_id: ItemId,
    pub token_price: u32,
    pub item_period: u32,
    pub level_limited: u32,
    pub quantity: u16, // If rechargeable encode unit price as u64
    pub max_per_slot: u16,
}

#[derive(ShroomPacket, Debug)]
pub struct OpenShopResp {
    pub npc_tmpl_id: NpcId,
    pub items: ShroomList16<ShopItem>,
}
with_opcode!(OpenShopResp, SendOpcodes::OpenShopDlg);

#[derive(ShroomPacketEnum, Debug)]
#[repr(u8)]
pub enum ShopResultResp {
    BuySuccess(()) = 0x0,
    BuyNoStock(()) = 0x1,
    BuyNoMoney(()) = 0x2,
    BuyUnknown(()) = 0x3,
    SellSuccess(()) = 0x4,
    SellNoStock(()) = 0x5,
    SellIncorrectRequest(()) = 0x6,
    SellUnkonwn(()) = 0x7,
    RechargeSuccess(()) = 0x8,
    RechargeNoStock(()) = 0x9,
    RechargeNoMoney(()) = 0xA,
    RechargeIncorrectRequest(()) = 0xB,
    RechargeUnknown(()) = 0xC,
    BuyNoToken(()) = 0xD,
    LimitLevelLess(u32) = 0xE,
    LimitLevelMore(u32) = 0xF,
    CantBuyAnymore(()) = 0x10,
    TradeBlocked(()) = 0x11,
    BuyLimit(()) = 0x12,
    ServerMsg(ShroomOption8<String>) = 0x13,
}
with_opcode!(ShopResultResp, SendOpcodes::ShopResult);

#[derive(ShroomPacket, Debug)]
pub struct ShopBuy {
    pub pos: u16,
    pub tmpl_id: u32,
    pub count: u16,
}

#[derive(ShroomPacket, Debug)]
pub struct ShopSell {
    pub slot: u16,
    pub tmpl_id: u32,
    pub count: u16,
}

#[derive(ShroomPacketEnum, Debug)]
#[repr(u8)]
pub enum ShopUserReq {
    Buy(ShopBuy) = 0,
    Sell(ShopSell) = 1,
    Recharge(u16) = 2,
    Close(()) = 3,
}
with_opcode!(ShopUserReq, RecvOpcodes::UserShopRequest);
