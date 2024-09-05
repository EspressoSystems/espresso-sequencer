// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (c) 2023 Espresso Systems (espressosys.com)
// This file is part of the Espresso Sequencer project.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If
// not, see <https://www.gnu.org/licenses/>.

// NOTE: DO NOT MODIFY! GENERATED BY SCRIPT VIA `cargo run --bin gen-vk-contract --release`.
pragma solidity ^0.8.0;

import { IPlonkVerifier } from "../../src/interfaces/IPlonkVerifier.sol";

/* solhint-disable no-inline-assembly */

library LightClientStateUpdateVKMock {
    function getVk() internal pure returns (IPlonkVerifier.VerifyingKey memory vk) {
        assembly {
            // domain size
            mstore(vk, 65536)
            // num of public inputs
            mstore(add(vk, 0x20), 8)

            // sigma0
            mstore(
                mload(add(vk, 0x40)),
                560292699158932187709021872580168940954720229866488907098569949929442242177
            )
            mstore(
                add(mload(add(vk, 0x40)), 0x20),
                10276961047134818946140351477110286957012577407066030912672129568036927281211
            )
            // sigma1
            mstore(
                mload(add(vk, 0x60)),
                5534691973404187513605300960613202894867859952192206103239775316623040666669
            )
            mstore(
                add(mload(add(vk, 0x60)), 0x20),
                10225857329627148128400064151472994355082888467374671967750208354120864174364
            )
            // sigma2
            mstore(
                mload(add(vk, 0x80)),
                8065316544719578387524358523310127191771093880959343997429567102465329547976
            )
            mstore(
                add(mload(add(vk, 0x80)), 0x20),
                1425838847304974874311980929869327314909669168012057463110471764544460175315
            )
            // sigma3
            mstore(
                mload(add(vk, 0xa0)),
                16002421545987331141742569210718550255048146570834778937216220592368925526073
            )
            mstore(
                add(mload(add(vk, 0xa0)), 0x20),
                34406611910962424253489125268856278032073593664804046642631011552934195946
            )
            // sigma4
            mstore(
                mload(add(vk, 0xc0)),
                11445079852804214826955859524249708877020358389341999422387741783694875036903
            )
            mstore(
                add(mload(add(vk, 0xc0)), 0x20),
                16806325692047749590626692981966359137981378903927139580947916716525859262646
            )

            // q1
            mstore(
                mload(add(vk, 0xe0)),
                2049866642812703568753016349662964168730767811710438483177174408835851039654
            )
            mstore(
                add(mload(add(vk, 0xe0)), 0x20),
                12520977158758014177420538652287908177326457213281949696327363920289084290701
            )
            // q2
            mstore(
                mload(add(vk, 0x100)),
                20582092476284202792941358171210260863634115075334011296845760245911069472015
            )
            mstore(
                add(mload(add(vk, 0x100)), 0x20),
                5041210168713491617697141661806293072927086571150532100480295734981490832235
            )
            // q3
            mstore(
                mload(add(vk, 0x120)),
                12744100466144754693458928600543682017448502839404097849586581378869048139272
            )
            mstore(
                add(mload(add(vk, 0x120)), 0x20),
                11764260783076322583704446914819686577350606893571049363874746256947123362836
            )
            // q4
            mstore(
                mload(add(vk, 0x140)),
                12704081410749559246476563784072861760155485913911911764179820885978638517991
            )
            mstore(
                add(mload(add(vk, 0x140)), 0x20),
                17722744854193267206230430932758110965125306922752471356921456991435364365754
            )

            // qM12
            mstore(
                mload(add(vk, 0x160)),
                6453231013362791895563067262275060970010067539446154128869944296180105433122
            )
            mstore(
                add(mload(add(vk, 0x160)), 0x20),
                1466658050526598062748535808722945599695400833147334646997674472844201846639
            )
            // qM34
            mstore(
                mload(add(vk, 0x180)),
                6582738640472984267346656690527485562185761247737941478591031706350232546802
            )
            mstore(
                add(mload(add(vk, 0x180)), 0x20),
                6919701491431094757031650741401030626916011721122969127785414082976995050894
            )

            // qO
            mstore(
                mload(add(vk, 0x1a0)),
                19789694753328811041438186861226325089790478291812954405045375609840541944055
            )
            mstore(
                add(mload(add(vk, 0x1a0)), 0x20),
                14159176289859745983359799835148658319127567280652459795489469350208679940139
            )
            // qC
            mstore(
                mload(add(vk, 0x1c0)),
                20664109960368631577285527855658840591636553141399541580771225677830051589235
            )
            mstore(
                add(mload(add(vk, 0x1c0)), 0x20),
                11723159022291546521707967932910777135698138237746209764135700920558934871080
            )
            // qH1
            mstore(
                mload(add(vk, 0x1e0)),
                605263757803570583600546503867857172101348083423514878846581641569710180355
            )
            mstore(
                add(mload(add(vk, 0x1e0)), 0x20),
                14469543004072753936869227821948184518584703336183789537429868167655429593124
            )
            // qH2
            mstore(
                mload(add(vk, 0x200)),
                2384667097398195228168221363677899853742498411390397795446900595952239300170
            )
            mstore(
                add(mload(add(vk, 0x200)), 0x20),
                10275888075018440494800367902299829818984504834118006945310306262240918085583
            )
            // qH3
            mstore(
                mload(add(vk, 0x220)),
                14833564191399885231304073197823073829536924923735425096809852677520137640772
            )
            mstore(
                add(mload(add(vk, 0x220)), 0x20),
                9327636621200489327971248548047951027037698837088692977367835437950105960107
            )
            // qH4
            mstore(
                mload(add(vk, 0x240)),
                4589838807130263610917801857966804873014329823411582139693177043027499096693
            )
            mstore(
                add(mload(add(vk, 0x240)), 0x20),
                4668230451870943729688296544612578377074246843166351853548960747414322359265
            )
            // qEcc
            mstore(
                mload(add(vk, 0x260)),
                1744575581311563417840597254368489349676731531441718204253650123654132132809
            )
            mstore(
                add(mload(add(vk, 0x260)), 0x20),
                4028482820459137632881945408189551152935873937068798814930695801575907701616
            )
            // g2LSB
            mstore(
                add(vk, 0x280), 0xb0838893ec1f237e8b07323b0744599f4e97b598b3b589bcc2bc37b8d5c41801
            )
            // g2MSB
            mstore(
                add(vk, 0x2A0), 0xc18393c0fa30fe4e8b038e357ad851eae8de9107584effe7c7f1f651b2010e26
            )
        }
    }
}
