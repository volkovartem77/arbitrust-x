
use crate::orderbook_stream::OrderbookStream;
use crate::utils::basic::{datetime_now_mcs, round, round_down, round_up};

const TAKER_FEE: f64 = 0.001;
const TAKER_FEE_PRC: f64 = 0.1;
const TAKER_FEE_RATIO: f64 = 0.999;
const PERCENT_PRECISION: u32 = 4;
const MIN_PROFIT_PERCENTAGE: f64 = 0.0;

impl OrderbookStream {
    fn get_amount_forward(&self) -> (f64, f64) {
        // Start by attempting to buy the full amount available in the TOKEN/USDT order book
        let mut token_purchased = self.orderbook_usdt.ask.amount;  // Get TOKEN/USDT Ask Amount
        let mut token_effective_after_fee = round_down(token_purchased * TAKER_FEE_RATIO, self.trading_params.token_usdt_base_asset_precision);

        // Round down to max precision to be able to place it in order book
        let mut token_effective_after_fee_for_sell = round_down(token_effective_after_fee, self.trading_params.token_btc_amount_precision);

        // Calculate the BTC resulting from selling this effective token at token/BTC, considering fees
        let btc_generated = round_down(token_effective_after_fee_for_sell * self.orderbook_btc.bid.price * TAKER_FEE_RATIO, self.trading_params.token_btc_base_asset_precision);

        // Round down to max precision to be able to place it in order book
        let btc_generated_for_sell = round_down(btc_generated, self.trading_params.btc_usdt_amount_precision);

        // Check if the generated BTC can be fully sold in the BTC/USDT market
        if btc_generated_for_sell > self.orderbook_btcusdt.bid.amount {
            // Adjust the token amount based on the BTC/USDT market limit
            token_effective_after_fee = round_up(round_up(self.orderbook_btcusdt.bid.amount / self.orderbook_btc.bid.price, self.trading_params.token_btc_base_asset_precision) / TAKER_FEE_RATIO, self.trading_params.token_btc_amount_precision);
            token_purchased = round_up(token_effective_after_fee / TAKER_FEE_RATIO, self.trading_params.token_usdt_amount_precision);
            token_effective_after_fee = round_down(token_purchased * TAKER_FEE_RATIO, self.trading_params.token_usdt_base_asset_precision);
            token_effective_after_fee_for_sell = round_down(token_effective_after_fee, self.trading_params.token_btc_amount_precision);
        }

        // Additionally, ensure the token amount does not exceed what can be sold in the token/BTC market
        if token_effective_after_fee_for_sell > self.orderbook_btc.bid.amount {
            // Adjust the token amount based on the token/BTC market limit
            token_purchased = round_up(self.orderbook_btc.bid.amount / TAKER_FEE_RATIO, self.trading_params.token_usdt_amount_precision);
        }

        // Ensure the recalculated token purchase does not exceed the initial token available
        token_purchased = token_purchased.min(self.orderbook_usdt.ask.amount);
        let usdt_required = round_up(token_purchased * self.orderbook_usdt.ask.price, self.trading_params.token_usdt_base_asset_precision);

        (token_purchased, usdt_required)
    }


    fn calc_forward_chain(&self, amount: &f64) -> (f64, f64) {
        // Forward chain is:
        // 1. TOKEN/USDT - BUY (Ask)
        // 2. TOKEN/BTC - SELL (Bid)
        // 3. BTC/USDT - SELL (Bid)


        let initial_usdt_required = round_up(amount * self.orderbook_usdt.ask.price, self.trading_params.token_usdt_quote_asset_precision);
        let ada_amount_after_fee = round_down(amount * TAKER_FEE_RATIO, self.trading_params.token_usdt_base_asset_precision);
        let ada_amount_tradable = round_down(ada_amount_after_fee, self.trading_params.token_btc_amount_precision);
        let btc_amount_generated = round_down(ada_amount_tradable * self.orderbook_btc.bid.price, self.trading_params.token_btc_base_asset_precision);
        let btc_amount_after_fee = round_down(btc_amount_generated * TAKER_FEE_RATIO, self.trading_params.token_btc_base_asset_precision);
        let btc_amount_tradable = round_down(btc_amount_after_fee, self.trading_params.btc_usdt_amount_precision);
        let usdt_amount_generated = round_down(btc_amount_tradable * self.orderbook_btcusdt.bid.price, self.trading_params.btc_usdt_quote_asset_precision);
        let usdt_amount_after_fee = round_down(usdt_amount_generated * TAKER_FEE_RATIO, self.trading_params.btc_usdt_quote_asset_precision);
        let max_profit_usdt = round(usdt_amount_after_fee - initial_usdt_required, self.trading_params.btc_usdt_quote_asset_precision.max(self.trading_params.token_usdt_quote_asset_precision));
        let profit_prc = round_down((max_profit_usdt / initial_usdt_required) * 100.0, PERCENT_PRECISION);

        (profit_prc, max_profit_usdt)
    }

    pub fn calc_opportunities(&self) {
        // println!("{} Calculation", datetime_now_mcs());

        // println!("Trading Parameters: {:?}", trading_params);
        // println!("Amount Precision - {}USDT: {}", trading_params.uppercase_token, trading_params.token_usdt_amount_precision);
        // println!("Amount Precision - {}BTC: {}", trading_params.uppercase_token, trading_params.token_btc_amount_precision);
        // println!("Amount Precision - BTCUSDT: {}", trading_params.btc_usdt_amount_precision);
        // println!("Base Asset Precision - {}USDT: {}", trading_params.uppercase_token, trading_params.token_usdt_base_asset_precision);
        // println!("Base Asset Precision - {}BTC: {}", trading_params.uppercase_token, trading_params.token_btc_base_asset_precision);


        // ----------------------------------------------------------------------------------------------------------------
        // ---------------------------------------------- F O R W A R D ---------------------------------------------------
        // ----------------------------------------------------------------------------------------------------------------

        let (amount, usdt_required) = self.get_amount_forward();
        let (profit_percentage, max_profit_usdt) = self.calc_forward_chain(&amount);

        if profit_percentage > MIN_PROFIT_PERCENTAGE {
            // println!("{} INFO Profit Percentage: {}%, Result amount = {} {}", datetime_now_mcs(), profit_percentage, amount, self.token_upper);
            println!("{} INFO New Arb: {}%, amount {} {}   [{}, {}, {}]", datetime_now_mcs(), profit_percentage,
                     amount, self.token_upper, self.orderbook_usdt.ask.price, self.orderbook_btc.bid.price, self.orderbook_btcusdt.bid.price);

            // TODO: продолжать здесь
            //  1. запустить на сарвере много пар на сканирование
            //  2. затем делать исполнение вилок
            //  3. возможно лучше перепроверить расчеты что бы быть на 100% уверенным в расчетах
        }
    }
}