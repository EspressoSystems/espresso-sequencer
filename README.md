<!--
 ~ Copyright (c) 2022 Espresso Systems (espressosys.com)
 ~ This file is part of the HotShot Query Service library.
 ~
 ~ This program is free software: you can redistribute it and/or modify it under the terms of the GNU
 ~ General Public License as published by the Free Software Foundation, either version 3 of the
 ~ License, or (at your option) any later version.
 ~ This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
 ~ even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 ~ General Public License for more details.
 ~ You should have received a copy of the GNU General Public License along with this program. If not,
 ~ see <https://www.gnu.org/licenses/>.
 -->

# HotShot Query Service

The HotShot Query Service is a minimal, generic query service that can be integrated into any
decentralized application running on the [HotShot](https://github.com/EspressoSystems/HotShot)
consensus layer. It provides all the features that HotShot itself expects of a query service (such
as providing consensus-related data for catchup and synchronization) as well as some
application-level features that deal only with consensus-related or application-agnostic data. In
addition, the query service is provided as an extensible library, which makes it easy to add
additional, application-specific features (such as a JSON-RPC service in the case of an EVM
application).

## Features
* Provides a query interface to a persistent store containing the history of a blockchain
  (represented as a chain of HotShot
  [leaves](https://hotshot.docs.espressosys.com/hotshot/data/struct.Leaf.html))
* Provides a query interface for validator status and metrics
* Integrates with HotShot to populate the persistent database
* Generic over application types (within a leaf, the block and block commitment can be any type the
  application specifieds)
* Transparently handles asynchronous data availability &mdash; block commitments are provided as
  soon as they are committed by consensus; block data is provided once it has disseminated from the
  data availability committee; missing data is fetched asynchronously from other nodes
* Serves as a source of data for HotShot catchup
* Easily extensible with application-specific functionality
