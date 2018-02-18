//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate stm32f40x;

use core::fmt::Write;

use cortex_m::asm;
use cortex_m_semihosting::hio;

//use stm32f40x::{gpiod, RCC};

fn delay() {
    let mut count = 0u32;
    loop {
        count += 1;
        if count > 1000000 {
            break;
        }
    }
}

fn main() {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, Rust!").unwrap();

    //let mut count = 0u32;
    loop {
        writeln!(stdout, "Те, кто выжил в ядерном огне,").unwrap();
        delay();
        writeln!(stdout, "назвали эту войну Судным Днём.").unwrap();
        delay();
        writeln!(stdout, "Но их ожидал новый кошмар:").unwrap();
        delay();
        writeln!(stdout, "война против машин.").unwrap();
        delay();
        writeln!(stdout, "Компьютер Скайнет, управлявший машинами,").unwrap();
        delay();
        writeln!(stdout, "послал в прошлое двух терминаторов.").unwrap();
        delay();
        writeln!(stdout, "Они должны были уничтожить того, кто").unwrap();
        delay();
        writeln!(stdout, "возглавил Сопротивление человечества.").unwrap();
        delay();
        writeln!(stdout, "Джона Коннора, моего сына.").unwrap();
        delay();
        writeln!(stdout, "Первый терминатор был запрограммирован").unwrap();
        delay();
        writeln!(stdout, "...Ещё до рождения Джона.").unwrap();
        delay();
        writeln!(stdout, "Он потерпел неудачу.").unwrap();
        delay();
        writeln!(stdout, "Второй был послать убить самого Джона,").unwrap();
        delay();
        writeln!(stdout, "когда тот был ещё ребенком.").unwrap();
        delay();
        writeln!(stdout, "Как и прежде, Сопротивлению удалось").unwrap();
        delay();
        writeln!(stdout, "отправить в прошлое одинокого воина...").unwrap();
        delay();
        writeln!(stdout, "...чтобы спасти Джона. Вопрос был только").unwrap();
        delay();
        writeln!(stdout, "в том, кто найдёт мальчика первым.").unwrap();
        delay();
        writeln!(stdout, "''Совпадает''").unwrap();
        delay();
        writeln!(stdout, "Мне нужны твоя одежда,").unwrap();
        delay();
        writeln!(stdout, "твоя обувь и мотоцикл.").unwrap();
        delay();
        writeln!(stdout, "Ты забыл сказать ''пожалуйста''.").unwrap();
        delay();
        writeln!(stdout, "Уберите его!").unwrap();
        delay();
        writeln!(stdout, "Выньте нож! Выньте нож!").unwrap();
        delay();
        writeln!(stdout, "Забирай.").unwrap();
        delay();
        writeln!(stdout, "Я не дам угонять мотоцикл").unwrap();
        delay();
        writeln!(stdout, "с моей стоянки.").unwrap();
        delay();
        writeln!(stdout, "Слезай, пока цел.").unwrap();
        delay();
        writeln!(stdout, "Ни шагу дальше.").unwrap();
        delay();
        writeln!(stdout, "Внимание, я 31-й.").unwrap();
        delay();
        writeln!(stdout, "Под мостом Санта-Фэ наблюдаются...").unwrap();
        delay();
        writeln!(stdout, "электрические разряды.").unwrap();
        delay();
        writeln!(stdout, "Продолжайте наблюдение.").unwrap();
        delay();
        writeln!(stdout, "''поиск.: Коннор Джон''").unwrap();
        delay();
        writeln!(stdout, "''фамилия - Коннор, имя - Джон,").unwrap();
        delay();
        writeln!(stdout, "пол - мужской, возраст - 10 лет''").unwrap();
        delay();
        writeln!(stdout, "''Приводы в полицию, нарушение порядка,").unwrap();
        delay();
        writeln!(stdout, "кража в магазине, вандализм''").unwrap();
        delay();
        writeln!(stdout, "''маты - Сара Коннор, отец - неизвестен,").unwrap();
        delay();
        writeln!(stdout, "опекуны - Тодд и Джейнелл Войт''").unwrap();
        delay();
        writeln!(stdout, "Джон. Джон, вернись").unwrap();
        delay();
        writeln!(stdout, "и прибери свой свинарник.").unwrap();
        delay();
        writeln!(stdout, "Джон!").unwrap();
        delay();
        writeln!(stdout, "Ну и зануда твоя опекунша.").unwrap();
        delay();
        writeln!(stdout, "Я сыта по горло этим мальчишкой.").unwrap();
        delay();
        writeln!(stdout, "Он мне даже не отвечает.").unwrap();
        delay();
        writeln!(stdout, "- Ты не в духе.").unwrap();
        delay();
        writeln!(stdout, "- Оторвись от экрана и помоги мне.").unwrap();
        delay();
        writeln!(stdout, "- Тодд!").unwrap();
        delay();
        writeln!(stdout, "- Ну что, что?").unwrap();
        delay();
        writeln!(stdout, "Он у себя в комнате").unwrap();
        delay();
        writeln!(stdout, "месяц не убирался.").unwrap();
        delay();
        writeln!(stdout, "Что за спешка? Сейчас разберусь.").unwrap();
        delay();
        writeln!(stdout, "- Садись. - Джон! Иди домой и делай,").unwrap();
        delay();
        writeln!(stdout, "что тебе мать говорит.").unwrap();
        delay();
        writeln!(stdout, "Она мне не мать, Тодд.").unwrap();
        delay();
        writeln!(stdout, "''Больница Пескадеро. Отделение больных").unwrap();
        delay();
        writeln!(stdout, "с опасными нарушениями психики''").unwrap();
        delay();
        writeln!(stdout, "А это очень интересный случай.").unwrap();
        delay();
        writeln!(stdout, "Мы наблюдаем её несколько лет.").unwrap();
        delay();
        writeln!(stdout, "Пациентке 29 лет. Она страдает...").unwrap();
        delay();
        writeln!(stdout, "...Острым шизофреническим").unwrap();
        delay();
        writeln!(stdout, "расстройством. Обычные симптомы:").unwrap();
        delay();
        writeln!(stdout, "...беспокойство, агрессивное").unwrap();

        //count += 1;
    }
    /*cortex_m::interrupt::free(|cs| {
        let gpioe = GPIOD.borrow(cs);
        let rcc = RCC.borrow(cs);

        rcc.ahbenr.modify(|_, w| w.iopden().enabled());
    });*/
    /*unsafe {
        let mut peripherals = stm32f40x::Peripherals::take().unwrap();
        peripherals.GPIOD.odr.write(|w| w.bits(14));
    }*/

    //peripherals.rcc.apb1enr.modify(|_, w| w.usart2en().enabled());
    //peripherals.rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
