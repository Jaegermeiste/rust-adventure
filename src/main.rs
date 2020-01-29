/**************************************************************************
 
    rust-adventure - a small adventure game written in Rust
    Copyright (C) 2019 Jason George

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

**************************************************************************/

mod input;
mod menu;
mod menuaction;
mod menupage;
mod game;
mod scrolltext;
use crate::input::*;
use crate::menu::*;
use crate::menuaction::*;
use crate::game::*;
use crate::scrolltext::*;
use std::io::{stdout, Write};
use crossterm::{
    terminal::{SetSize},
    execute,
};

pub static DEFAULT_TERMINAL_WIDTH   : u16 = 80;
pub static DEFAULT_TERMINAL_HEIGHT  : u16 = 30; 

fn main() {
    let input                       = Input::new();
    let mut scrolltext              = ScrollText::new(&input);
    let mut menu_system             = Menu::new(&input);
    let main_menu_index             = menu_system.add_page("Main Menu");
    let about_page_index            = menu_system.add_page("About");
    let main_menu_new_game_option	= menu_system.add_action_to_page(main_menu_index,   MenuAction::new("Start New Game",	            MenuActionType::Selector));
    let main_menu_about_option	    = menu_system.add_action_to_page(main_menu_index,   MenuAction::new("About",	                    MenuActionType::Selector));
    let main_menu_quit_option		= menu_system.add_action_to_page(main_menu_index,   MenuAction::new("Exit Program",                 MenuActionType::ExitMenu));
    let about_menu_copyright_option	= menu_system.add_action_to_page(about_page_index,  MenuAction::new("Show Copyright Details",	    MenuActionType::Selector));
    let about_menu_fair_use_option	= menu_system.add_action_to_page(about_page_index,  MenuAction::new("Show Fair Use Disclaimer",	    MenuActionType::Selector));
    let about_menu_warranty_option	= menu_system.add_action_to_page(about_page_index,  MenuAction::new("Show Warranty Details",	    MenuActionType::Selector));
    let about_menu_redist_option	= menu_system.add_action_to_page(about_page_index,  MenuAction::new("Show Redistibution Details",   MenuActionType::Selector));
    let about_menu_license_option	= menu_system.add_action_to_page(about_page_index,  MenuAction::new("Show License",                 MenuActionType::Selector));
    let about_menu_credits_option	= menu_system.add_action_to_page(about_page_index,  MenuAction::new("Show Credits and Attribution", MenuActionType::Selector));
    let about_menu_main_menu_option	= menu_system.add_action_to_page(about_page_index,  MenuAction::new("Return to Main Menu",	        MenuActionType::ExitMenu));

    // Set terminal size
    let mut _result = execute!(stdout(), SetSize(DEFAULT_TERMINAL_WIDTH, DEFAULT_TERMINAL_HEIGHT));
    
    // License attribution
    println!(" rust-adventure (C) 2019 Jason George");
    println!(" This program comes with ABSOLUTELY NO WARRANTY; for details select 'About->Show Warranty Details' at the menu.");
    println!(" This is free software, and you are welcome to redistribute it under certain conditions; select 'About->Show Redistibution Details' at the menu for details.");

    // Display the Main Menu
	let mut main_menu_selection = menu_system.run_page(main_menu_index);

	// Loop so long as we haven't selected to quit
	while main_menu_selection != main_menu_quit_option {

		if main_menu_selection == main_menu_new_game_option	{
			let game = Game::new(&input);

			game.run();
        }
        else if main_menu_selection == main_menu_about_option {
            let mut about_menu_selection = menu_system.run_page(about_page_index);

            while about_menu_selection != about_menu_main_menu_option {  
                let (_width, height) = crossterm::terminal::size().unwrap();   

                if about_menu_selection == about_menu_copyright_option {
                    let copyright_text = 
                    "rust-adventure\n(C) 2019 Jason George\n\nThis program is made available by the author to the open source community under the terms of the GNU General Public License, Version 3.\n\nPortions of this work constitute a parody under the Fair Use Doctrine.\nThis transformative work constitutes a fair-use of any copyrighted material as provided for in Section 107 of US Copyright Law.\n".to_string();
                
                    scrolltext.print_text_range(copyright_text, 0, height as u32);
                }
                else if about_menu_selection == about_menu_fair_use_option {
                    let fair_use_text = 
                    "Disclaimer and Fair Use Statement\n\nThis application may contain or reference copyrighted material, the use of which may not have been specifically authorized by the copyright owner. This material is used in a parodic manner. The material contained in this application is distributed without profit for research and educational purposes. Only small portions of the original work are being used and those could not be used easily to duplicate the original work.\n\nThis should constitute a ‘fair use’ of any such copyrighted material (referenced and provided for in section 107 of the US Copyright Law).\n\nIf you wish to use any copyrighted material from this application for purposes of your own that go beyond ‘fair use’, you must obtain expressed permission from the copyright owner.\n".to_string();
                
                    scrolltext.print_text_range(fair_use_text, 0, height as u32);
                }
                else if about_menu_selection == about_menu_warranty_option {
                    let warranty_text = 
                    "15. Disclaimer of Warranty.\nTHERE IS NO WARRANTY FOR THE PROGRAM, TO THE EXTENT PERMITTED BY APPLICABLE LAW. EXCEPT WHEN OTHERWISE STATED IN WRITING THE COPYRIGHT HOLDERS AND/OR OTHER PARTIES PROVIDE THE PROGRAM \"AS IS\" WITHOUT WARRANTY OF ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE ENTIRE RISK AS TO THE QUALITY AND PERFORMANCE OF THE PROGRAM IS WITH YOU. SHOULD THE PROGRAM PROVE DEFECTIVE, YOU ASSUME THE COST OF ALL NECESSARY SERVICING, REPAIR OR CORRECTION.\n\n16. Limitation of Liability.\nIN NO EVENT UNLESS REQUIRED BY APPLICABLE LAW OR AGREED TO IN WRITING WILL ANY COPYRIGHT HOLDER, OR ANY OTHER PARTY WHO MODIFIES AND/OR CONVEYS THE PROGRAM AS PERMITTED ABOVE, BE LIABLE TO YOU FOR DAMAGES, INCLUDING ANY GENERAL, SPECIAL, INCIDENTAL OR CONSEQUENTIAL DAMAGES ARISING OUT OF THE USE OR INABILITY TO USE THE PROGRAM (INCLUDING BUT NOT LIMITED TO LOSS OF DATA OR DATA BEING RENDERED INACCURATE OR LOSSES SUSTAINED BY YOU OR THIRD PARTIES OR A FAILURE OF THE PROGRAM TO OPERATE WITH ANY OTHER PROGRAMS), EVEN IF SUCH HOLDER OR OTHER PARTY HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.\n".to_string();
                
                    scrolltext.print_text_range(warranty_text, 0, height as u32);
                }
                else if about_menu_selection == about_menu_redist_option {
                    let redist_text =
                    "4. Conveying Verbatim Copies.\nYou may convey verbatim copies of the Program's source code as you receive it, in any medium, provided that you conspicuously and appropriately publish on each copy an appropriate copyright notice; keep intact all notices stating that this License and any non-permissive terms added in accord with section 7 apply to the code; keep intact all notices of the absence of any warranty; and give all recipients a copy of this License along with the Program.\nYou may charge any price or no price for each copy that you convey, and you may offer support or warranty protection for a fee.\n\n5. Conveying Modified Source Versions.\nYou may convey a work based on the Program, or the modifications to produce it from the Program, in the form of source code under the terms of section 4, provided that you also meet all of these conditions:\na) The work must carry prominent notices stating that you modified it, and giving a relevant date.\nb) The work must carry prominent notices stating that it is released under this License and any conditions added under section 7. This requirement modifies the requirement in section 4 to \"keep intact all notices\".\nc) You must license the entire work, as a whole, under this License to anyone who comes into possession of a copy. This License will therefore apply, along with any applicable section 7 additional terms, to the whole of the work, and all its parts, regardless of how they are packaged. This License gives no permission to license the work in any other way, but it does not invalidate such permission if you have separately received it.\nd) If the work has interactive user interfaces, each must display Appropriate Legal Notices; however, if the Program has interactive interfaces that do not display Appropriate Legal Notices, your work need not make them do so.\nA compilation of a covered work with other separate and independent works, which are not by their nature extensions of the covered work, and which are not combined with it such as to form a larger program, in or on a volume of a storage or distribution medium, is called an \"aggregate\" if the compilation and its resulting copyright are not used to limit the access or legal rights of the compilation's users beyond what the individual works permit. Inclusion of a covered work in an aggregate does not cause this License to apply to the other parts of the aggregate.\n\n6. Conveying Non-Source Forms.\nYou may convey a covered work in object code form under the terms of sections 4 and 5, provided that you also convey the machine-readable Corresponding Source under the terms of this License, in one of these ways:\na) Convey the object code in, or embodied in, a physical product (including a physical distribution medium), accompanied by the Corresponding Source fixed on a durable physical medium customarily used for software interchange.\nb) Convey the object code in, or embodied in, a physical product (including a physical distribution medium), accompanied by a written offer, valid for at least three years and valid for as long as you offer spare parts or customer support for that product model, to give anyone who possesses the object code either (1) a copy of the Corresponding Source for all the software in the product that is covered by this License, on a durable physical medium customarily used for software interchange, for a price no more than your reasonable cost of physically performing this conveying of source, or (2) access to copy the Corresponding Source from a network server at no charge.\nc) Convey individual copies of the object code with a copy of the written offer to provide the Corresponding Source. This alternative is allowed only occasionally and noncommercially, and only if you received the object code with such an offer, in accord with subsection 6b.\nd) Convey the object code by offering access from a designated place (gratis or for a charge), and offer equivalent access to the Corresponding Source in the same way through the same place at no further charge. You need not require recipients to copy the Corresponding Source along with the object code. If the place to copy the object code is a network server, the Corresponding Source may be on a different server (operated by you or a third party) that supports equivalent copying facilities, provided you maintain clear directions next to the object code saying where to find the Corresponding Source. Regardless of what server hosts the Corresponding Source, you remain obligated to ensure that it is available for as long as needed to satisfy these requirements.\ne) Convey the object code using peer-to-peer transmission, provided you inform other peers where the object code and Corresponding Source of the work are being offered to the general public at no charge under subsection 6d.\nA separable portion of the object code, whose source code is excluded from the Corresponding Source as a System Library, need not be included in conveying the object code work.\n\nA \"User Product\" is either (1) a \"consumer product\", which means any tangible personal property which is normally used for personal, family, or household purposes, or (2) anything designed or sold for incorporation into a dwelling. In determining whether a product is a consumer product, doubtful cases shall be resolved in favor of coverage. For a particular product received by a particular user, \"normally used\" refers to a typical or common use of that class of product, regardless of the status of the particular user or of the way in which the particular user actually uses, or expects or is expected to use, the product. A product is a consumer product regardless of whether the product has substantial commercial, industrial or non-consumer uses, unless such uses represent the only significant mode of use of the product.\n\n\"Installation Information\" for a User Product means any methods, procedures, authorization keys, or other information required to install and execute modified versions of a covered work in that User Product from a modified version of its Corresponding Source. The information must suffice to ensure that the continued functioning of the modified object code is in no case prevented or interfered with solely because modification has been made.\n\nIf you convey an object code work under this section in, or with, or specifically for use in, a User Product, and the conveying occurs as part of a transaction in which the right of possession and use of the User Product is transferred to the recipient in perpetuity or for a fixed term (regardless of how the transaction is characterized), the Corresponding Source conveyed under this section must be accompanied by the Installation Information. But this requirement does not apply if neither you nor any third party retains the ability to install modified object code on the User Product (for example, the work has been installed in ROM).\n\nThe requirement to provide Installation Information does not include a requirement to continue to provide support service, warranty, or updates for a work that has been modified or installed by the recipient, or for the User Product in which it has been modified or installed. Access to a network may be denied when the modification itself materially and adversely affects the operation of the network or violates the rules and protocols for communication across the network.\n\nCorresponding Source conveyed, and Installation Information provided, in accord with this section must be in a format that is publicly documented (and with an implementation available to the public in source code form), and must require no special password or key for unpacking, reading or copying.\n".to_string();

                    scrolltext.print_text_range(redist_text, 0, height as u32);
                }
                else if about_menu_selection == about_menu_license_option {
                    scrolltext.print_file_range("LICENSE.txt", 0, height as u32);
                }
                else if about_menu_selection == about_menu_credits_option {
                    let attribution_text =
                    "Credits\n\n2017-2020 Jason George\n\nThanks to my family, who (barely) puts up with me writing this stuff all day.\n\n\nThanks to all the game developers and pop culture creatives whose tireless crunch has added to the enjoyment of billions and inspired this work.\nThese inspirations include (but are not limited to):\n - Muse Software\n - id Software\n - Valve Software\n - Red Bull\n - Bethesda Softworks\n - Ubisoft Entertainment\n - Netflix\n - South Park Studios\n - Blizzard Entertainment\n - BioWare\n - Nintendo\n\n\nTo anyone I missed explicitly attributing, thank you!\n\n".to_string();

                    scrolltext.print_text_range(attribution_text, 0, height as u32);
                }

                about_menu_selection = menu_system.run_page(about_page_index);
            }
        }
				
		// after running the function, return to the menu
		main_menu_selection = menu_system.run_page(main_menu_index);
	}
}
